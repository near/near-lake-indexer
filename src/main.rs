use std::sync::Arc;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Endpoint, Region};
use clap::Parser;
use configs::{Opts, SubCommand};
use futures::StreamExt;
use tokio::sync::Mutex;
use tracing_subscriber::EnvFilter;

mod configs;
mod utils;

const INDEXER: &str = "near_lake";

#[derive(Debug, Clone)]
struct Stats {
    pub block_heights_processing: std::collections::BTreeSet<u64>,
    pub blocks_processed_count: u64,
    pub last_processed_block_height: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            block_heights_processing: std::collections::BTreeSet::new(),
            blocks_processed_count: 0,
            last_processed_block_height: 0,
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (sending telemetry and downloading genesis)
    openssl_probe::init_ssl_cert_env_vars();
    init_tracing();

    let opts: Opts = Opts::parse();

    let home_dir = opts.home.unwrap_or_else(near_indexer::get_default_home);

    match opts.subcmd {
        SubCommand::Run(args) => {
            tracing::info!(
                target: INDEXER,
                "NEAR Lake v{} starting...",
                env!("CARGO_PKG_VERSION")
            );

            let system = actix::System::new();
            system.block_on(async move {
                let indexer_config = args.clone().to_indexer_config(home_dir);
                let indexer = near_indexer::Indexer::new(indexer_config)
                    .expect("Failed to initialize the Indexer");

                // Regular indexer process starts here
                let stream = indexer.streamer();
                let view_client = indexer.client_actors().0;

                let stats: Arc<Mutex<Stats>> = Arc::new(Mutex::new(Stats::new()));

                actix::spawn(lake_logger(Arc::clone(&stats), view_client));

                listen_blocks(
                    stream,
                    args.endpoint,
                    args.bucket,
                    args.region,
                    args.fallback_region,
                    args.concurrency,
                    Arc::clone(&stats),
                )
                .await;

                actix::System::current().stop();
            });
            system.run().unwrap();
        }
        SubCommand::Init(config) => near_indexer::init_configs(
            &home_dir,
            config.chain_id,
            config.account_id.map(|account_id_string| {
                near_indexer::near_primitives::types::AccountId::try_from(account_id_string)
                    .expect("Received accound_id is not valid")
            }),
            config.test_seed.as_ref().map(AsRef::as_ref),
            config.num_shards,
            config.fast,
            config.genesis.as_ref().map(AsRef::as_ref),
            config.download_genesis,
            config.download_genesis_url.as_ref().map(AsRef::as_ref),
            config
                .donwload_genesis_records_url
                .as_ref()
                .map(AsRef::as_ref),
            config.download_config,
            config.download_config_url.as_ref().map(AsRef::as_ref),
            config.boot_nodes.as_ref().map(AsRef::as_ref),
            config.max_gas_burnt_view,
        )
        .expect("Failed to initialize the node config files"),
    }
}

async fn lake_logger(
    stats: Arc<Mutex<Stats>>,
    view_client: actix::Addr<near_client::ViewClientActor>,
) {
    let interval_secs = 10;
    let mut prev_blocks_processed_count: u64 = 0;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
        let stats_lock = stats.lock().await;
        let stats_copy = stats_lock.clone();
        drop(stats_lock);

        let block_processing_speed: f64 = ((stats_copy.blocks_processed_count
            - prev_blocks_processed_count) as f64)
            / (interval_secs as f64);

        let time_to_catch_the_tip_duration = if block_processing_speed > 0.0 {
            if let Ok(block_height) = utils::fetch_latest_block(&view_client).await {
                Some(std::time::Duration::from_millis(
                    (((block_height - stats_copy.last_processed_block_height) as f64
                        / block_processing_speed)
                        * 1000f64) as u64,
                ))
            } else {
                None
            }
        } else {
            None
        };

        tracing::info!(
            target: INDEXER,
            "# {} | Blocks processing: {}| Blocks done: {}. Bps {:.2} b/s {}",
            stats_copy.last_processed_block_height,
            stats_copy.block_heights_processing.len(),
            stats_copy.blocks_processed_count,
            block_processing_speed,
            if let Some(duration) = time_to_catch_the_tip_duration {
                format!(
                    " | {} to catch up the tip",
                    humantime::format_duration(duration)
                )
            } else {
                "".to_string()
            }
        );
        prev_blocks_processed_count = stats_copy.blocks_processed_count;
    }
}

async fn listen_blocks(
    stream: tokio::sync::mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
    endpoint: Option<http::Uri>,
    bucket: String,
    region: String,
    fallback_region: String,
    concurrency: std::num::NonZeroU16,
    stats: Arc<Mutex<Stats>>,
) {
    let region_provider = RegionProviderChain::first_try(Some(region).map(Region::new))
        .or_default_provider()
        .or_else(Region::new(fallback_region));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let mut s3_conf = aws_sdk_s3::config::Builder::from(&shared_config);
    // Owerride S3 endpoint in case you want to use custom solution
    // like Minio or Localstack as a S3 compatible storage
    if let Some(s3_endpoint) = endpoint {
        s3_conf = s3_conf.endpoint_resolver(Endpoint::immutable(s3_endpoint.clone()));
        tracing::info!(target: INDEXER, "Custom S3 endpoint used: {}", s3_endpoint);
    }

    let client = Client::from_conf(s3_conf.build());

    let mut handle_messages = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| {
            handle_message(
                &client,
                streamer_message,
                bucket.clone(),
                Arc::clone(&stats),
            )
        })
        .buffer_unordered(usize::from(concurrency.get()));

    while let Some(_handle_message) = handle_messages.next().await {}
}

async fn handle_message(
    client: &Client,
    streamer_message: near_indexer_primitives::StreamerMessage,
    bucket: String,
    stats: Arc<Mutex<Stats>>,
) -> anyhow::Result<()> {
    let block_height = streamer_message.block.header.height;
    let mut stats_lock = stats.lock().await;
    stats_lock.block_heights_processing.insert(block_height);
    drop(stats_lock);

    let base_key = format!("{:0>12}", streamer_message.block.header.height);

    // Block
    let block_json = serde_json::to_value(streamer_message.block)
        .expect("Failed to serializer BlockView to JSON");
    put_object_or_retry(
        client.clone(),
        bucket.clone(),
        block_json,
        format!("{}/block.json", base_key).to_string(),
    )
    .await;

    // Shards
    for shard in streamer_message.shards.iter() {
        let key = format!("{}/shard_{}.json", base_key, shard.shard_id);
        let shard_json =
            serde_json::to_value(shard).expect("Failed to serialize IndexerShard to JSON");
        put_object_or_retry(client.clone(), bucket.clone(), shard_json, key).await;
    }
    let mut stats_lock = stats.lock().await;
    stats_lock.block_heights_processing.remove(&block_height);
    stats_lock.blocks_processed_count += 1;
    stats_lock.last_processed_block_height = block_height;
    drop(stats_lock);
    Ok(())
}

// Saves an object to a bucket or retries forever. Aborts the entire process if credentials are missing
async fn put_object_or_retry(
    client: Client,
    bucket: String,
    content: serde_json::Value,
    filename: String,
) {
    loop {
        let body = ByteStream::from(content.clone().to_string().as_bytes().to_vec());
        match put_object(&client, &bucket, body, filename.as_str()).await {
            Ok(_) => break,
            Err(err) => {
                // We haven't found the way to check credentials before the request has been sent
                // This is the weird yet working solution to throw an error if we got
                // missing credentials error
                if let aws_smithy_http::result::SdkError::ConstructionFailure(box_error) = err {
                    if box_error.to_string() == *"No credentials in the property bag".to_string() {
                        tracing::error!(target: INDEXER, "No credentials in the property bag");
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                }
                tracing::warn!(
                    target: INDEXER,
                    "Failed to put {} to S3, retrying",
                    &filename
                );
            }
        }
    }
}

// Adds an object to a bucket
async fn put_object(
    client: &Client,
    bucket: &str,
    body: ByteStream,
    filename: &str,
) -> Result<(), aws_smithy_http::result::SdkError<aws_sdk_s3::error::PutObjectError>> {
    client
        .put_object()
        .bucket(bucket)
        .body(body)
        .key(filename)
        .send()
        .await?;

    Ok(())
}

fn init_tracing() {
    let mut env_filter = EnvFilter::new(
        "tokio_reactor=info,near=info,stats=info,telemetry=info,indexer=info,near_lake=info,aggregated=info",
    );

    if let Ok(rust_log) = std::env::var("RUST_LOG") {
        if !rust_log.is_empty() {
            for directive in rust_log.split(',').filter_map(|s| match s.parse() {
                Ok(directive) => Some(directive),
                Err(err) => {
                    eprintln!("Ignoring directive `{}`: {}", s, err);
                    None
                }
            }) {
                env_filter = env_filter.add_directive(directive);
            }
        }
    }

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}
