# near-lake

**NB! The project is in early stage of development and shouldn't be used in production environmental yet**

NEAR Lake is an indexer built on top of [NEAR Indexer microframework](https://github.com/nearprotocol/nearcore/tree/master/chain/indexer)
to watch the network and store all the events as JSON files on AWS S3.

## Concept

We used to have [NEAR Indexer for Explorer](https://github.com/near/near-indexer-for-explorer) that was watching for 
the network and stored all the events to PostgreSQL database. PostgreSQL became the main bottleneck for us. After some
brainstorming sessions and researches we decided to go with [SingleStore](https://www.singlestore.com/) database. 

Knowing the fact that [NEAR Explorer](https://explorer.near.org) is not the only project that uses the Indexer for Explorer's 
database, we wanted to come up with the concept that will allow us to cover even more projects that can benefit from the data 
from NEAR Protocol.

That's why we decided to store the data from the blockchain as JSON files on AWS S3 bucket that can be used
as a data source for different projects. 

As "Indexer for Explorer Remake" project we are going to have `near-lake` as a data writer. There's going to be
another project that will read from AWS S3 bucket and will store all the data in SingleStore database. This
will replace NEAR Indexer for Explorer PostgreSQL database at some moment and will become the main
source for NEAR Explorer.

## How to start

The final setup consists of the following components:
* AWS S3 Bucket as a storage
* NEAR Lake binary that operates as a regular NEAR Protocol peer-to-peer node, so you will operate it as 
  any other [Regular/RPC Node in NEAR](https://docs.near.org/docs/develop/node/rpc/hardware-rpc)

### Prepare Development Environment

Before you proceed, make sure you have the following software installed:
* [Rust compiler](https://rustup.rs/) of the version that is mentioned in `rust-toolchain` file in the root of
  [nearcore](https://github.com/nearprotocol/nearcore) project.
* Ensure you have [AWS Credentials configured](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
    From AWS Docs:

  > For example, the files generated by the AWS CLI for a default profile configured with aws configure looks similar to the following.
  >
  > ~/.aws/credentials
  > ```
  > [default]
  > aws_access_key_id=AKIAIOSFODNN7EXAMPLE
  > aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
  > ```  
  >  
  > ~/.aws/config
  > ```
  > [default]
  > region=us-west-2
  > output=json
  > ```  

### Compile NEAR Lake

```bash
$ cargo build --release
```

### Configure NEAR Lake

To connect NEAR Lake to the specific chain you need to have necessary configs, you can generate it as follows:

```bash
$ ./target/release/near-lake --home-dir ~/.near/testnet init --chain-id testnet --download-config --download-genesis
```

The above code will download the official genesis config and generate necessary configs. You can replace `testnet` in the command above to different network ID (`betanet`, `mainnet`).

**NB!** According to changes in `nearcore` config generation we don't fill all the necessary fields in the config file.
While this issue is open https://github.com/nearprotocol/nearcore/issues/3156 you need to download config you want and replace the generated one manually.
- [testnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json)
- [betanet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json)
- [mainnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json)

Configs for the specified network are in the `--home-dir` provided folder. We need to ensure that NEAR Lake follows
all the necessary shards, so `"tracked_shards"` parameters in `~/.near/testnet/config.json` needs to be configured properly.
For example, with a single shared network, you just add the shard #0 to the list:

```
...
"tracked_shards": [0],
...
```

### Run NEAR Lake

Commands to run NEAR Lake

| binary 	| Command 	| Key/Subcommand               	| Required/Default                                                 	| Responsible for                                                                                                                                                                                                                                                                                                                                                         	|
|--------	|---------	|------------------------------	|------------------------------------------------------------------	|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|
|        	|         	| `--home-dir`                 	| Default <br>`~/.near`                                            	| Tells the node where too look for necessary files: <br>`config.json`<br>, <br>`genesis.json`<br>, <br>`node_key.json`<br>, and <br>`data`<br> folder                                                                                                                                                                                                                    	|
|        	| `init`  	|                              	|                                                                  	| Tells the node to generate config files in `--home-dir`                                                                                                                                                                                                                                                                                                                 	|
|        	|         	| `--chain-id`                 	| Required<br><br>  * `localnet`<br>  * `testnet`<br>  * `mainnet` 	| Defines the chain to generate config files for                                                                                                                                                                                                                                                                                                                          	|
|        	|         	| `--download-config`          	| Optional                                                         	| If provided tells the node to download `config.json` from the public URL. You can download them manually<br><br> - [testnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json)<br> - [mainnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json)      	|
|        	|         	| `--download-genesis`         	| Optional                                                         	| If provided tells the node to download `genesis.json` from the public URL. You can download them manually<br><br> - [testnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/genesis.json)<br> - [mainnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/genesis.json) 	|
|        	|         	| TODO:<br>Other `neard` keys  	|                                                                  	|                                                                                                                                                                                                                                                                                                                                                                         	|
|        	| `run`   	|                              	|                                                                  	| Runs the node                                                                                                                                                                                                                                                                                                                                                           	|
|        	|         	| `--bucket`                   	| Required                                                         	| AWS S3 Bucket name                                                                                                                                                                                                                                                                                                                                                      	|
|        	|         	| `--region`                   	| Required                                                         	| AWS S3 Bucket region                                                                                                                                                                                                                                                                                                                                                    	|
|        	|         	| `--stream-while-syncing`     	| Optional                                                         	| If provided Indexer streams blocks while they appear on the node instead of waiting the node to be fully synced                                                                                                                                                                                                                                                         	|
|        	|         	| `--concurrency`              	| Default 1                                                        	| Defines the concurrency for the process of saving block data to AWS S3                                                                                                                                                                                                                                                                                                  	|
|        	|         	| `sync-from-latest`           	| One of the `sync-` subcommands is required                       	| Tells the node to start indexing from the latest block in the network                                                                                                                                                                                                                                                                                                   	|
|        	|         	| `sync-from-interruption`     	| One of the `sync-` subcommands is required                       	| Tells the node to start indexing from the block the node was interrupted on (if it is a first start it will fallback to `sync-from-latest`)                                                                                                                                                                                                                             	|
|        	|         	| `sync-from-block --height N` 	| One of the <br>`sync-`<br> subcommands is required               	| Tells the node to start indexing from the specified block height `N` (**Ensure** you node data has the block you want to start from)                                                                                                                                                                                                                                    	|

```bash
$ ./target/release/near-lake --home-dir ~/.near/testnet run --stream-while-syncing --concurrency 50 sync-from-latest
```

After the network is synced, you should see logs of every block height currently received by NEAR Lake.


## Syncing

Whenever you run NEAR Lake for any network except localnet you'll need to sync with the network. 
This is required because it's a natural behavior of `nearcore` node and NEAR Lake is a wrapper 
for the regular `nearcore` node. In order to work and index the data your node must be synced 
with the network. This process can take a while, so we suggest to download a fresh backup of 
the `data` folder and put it in you `--home-dir` of your choice (by default it is `~/.near`)

Running your NEAR Lake node on top of a backup data will reduce the time of syncing process 
because your node will download only the data after the backup was cut and it takes reasonable amount time.

All the backups can be downloaded from the public S3 bucket which contains latest daily snapshots:

* [Recent 5-epoch Mainnet data folder](https://near-protocol-public.s3-accelerate.amazonaws.com/backups/mainnet/rpc/data.tar)
* [Recent 5-epoch Testnet data folder](https://near-protocol-public.s3-accelerate.amazonaws.com/backups/testnet/rpc/data.tar)


## Running NEAR Lake as an archival node

It's not necessary but in order to index everything in the network it is better to do it from the genesis. 
`nearcore` node is running in non-archival mode by default. That means that the node keeps data only 
for [5 last epochs](https://docs.near.org/docs/concepts/epoch). In order to index data from the genesis 
we need to turn the node in archival mode.

To do it we need to update `config.json` located in `--home-dir` (by default it is `~/.near`).

Find next keys in the config and update them as following:

```json
{
  ...
  "archive": true,
  "tracked_shards": [0],
  ...
}
```

The syncing process in archival mode can take a lot of time, so it's better to download a backup provided by NEAR 
and put it in your `data` folder. After that your node will need to sync only missing data and it should take 
reasonable time.

All the backups can be downloaded from the public S3 bucket which contains the latest daily snapshots:

* [Archival Mainnet data folder](https://near-protocol-public.s3-accelerate.amazonaws.com/backups/mainnet/archive/data.tar)
* [Archival Testnet data folder](https://near-protocol-public.s3-accelerate.amazonaws.com/backups/testnet/archive/data.tar)

See https://docs.near.org/docs/roles/integrator/exchange-integration#running-an-archival-node for reference