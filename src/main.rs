use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client, Region};
use clap::Parser;
use configs::{Opts, SubCommand};
use futures::StreamExt;
use tracing_subscriber::EnvFilter;

mod configs;

const INDEXER: &str = "near_lake";

fn main() {
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

                listen_blocks(stream, args.bucket, args.region, args.concurrency).await;

                actix::System::current().stop();
            });
            system.run().unwrap();
        }
        SubCommand::Init(config) => near_indexer::init_configs(
            &home_dir,
            config.chain_id.as_ref().map(AsRef::as_ref),
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
            config.download_config,
            config.download_config_url.as_ref().map(AsRef::as_ref),
            config.boot_nodes.as_ref().map(AsRef::as_ref),
            config.max_gas_burnt_view,
        )
        .expect("Failed to initialize the node config files"),
    }
}

async fn listen_blocks(
    stream: tokio::sync::mpsc::Receiver<near_indexer_primitives::StreamerMessage>,
    bucket: String,
    region: String,
    concurrency: std::num::NonZeroU16,
) {
    let region_provider = RegionProviderChain::first_try(Some(region).map(Region::new))
        .or_default_provider()
        .or_else(Region::new("eu-central-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let mut handle_messages = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| {
            tracing::info!(
                target: INDEXER,
                "Block height {}",
                &streamer_message.block.header.height
            );
            handle_message(&client, streamer_message, bucket.clone())
        })
        .buffer_unordered(usize::from(concurrency.get()));

    while let Some(_handle_message) = handle_messages.next().await {}
}

async fn handle_message(
    client: &Client,
    streamer_message: near_indexer_primitives::StreamerMessage,
    bucket: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_key = streamer_message.block.header.height.to_string();

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
                // This is the weird yet working solution to fail entire application if we got
                // missing credentials error
                match err {
                    aws_smithy_http::result::SdkError::ConstructionFailure(box_error) => {
                        if box_error.to_string()
                            == String::from("No credentials in the property bag")
                        {
                            tracing::error!(target: INDEXER, "No credentials in the property bag");
                            std::process::abort();
                        }
                    }
                    _ => {}
                };
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
