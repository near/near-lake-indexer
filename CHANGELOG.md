# Changelog

## 2.3.0-rc.3

* Upgrade Indexer Framework to be based on [nearcore 2.3.0-rc.3](https://github.com/near/nearcore/releases/tag/2.3.0-rc.3)

## 2.3.0-rc.2

* Upgrade Indexer Framework to be based on [nearcore 2.3.0-rc.2](https://github.com/near/nearcore/releases/tag/2.3.0-rc.2)

## 2.3.0-rc.1

* Upgrade Indexer Framework to be based on [nearcore 2.3.0-rc.1](https://github.com/near/nearcore/releases/tag/2.3.0-rc.1)
* Bump the minimal supported Rust version to `1.81.0` (following the `nearcore` master branch)
* Adopt the changes from the `nearcore` around how the typical configs are downloaded now (see [nearcore#12070](https://github.com/near/nearcore/issues/12070))

## 2.2.1

* Upgrade Indexer Framework to be based on [nearcore 2.2.1](https://github.com/near/nearcore/releases/tag/2.2.1)

## 2.2.0

* Upgrade Indexer Framework to be based on [nearcore 2.2.0](https://github.com/near/nearcore/releases/tag/2.2.0)

## 2.2.0-rc.1

* Upgrade Indexer Framework to be based on [nearcore 2.2.0-rc.1](https://github.com/near/nearcore/releases/tag/2.2.0-rc.1)

## 2.1.1

* Upgrade Indexer Framework to be based on [nearcore 2.1.1](https://github.com/near/nearcore/releases/tag/2.1.1)

## 2.1.0-rc.1

* Upgrade Indexer Framework to be based on [nearcore 2.1.0-rc.1](https://github.com/near/nearcore/releases/tag/2.1.0-rc.1)
* Bump the minimal supported Rust version to `1.79.0` (following the `nearcore` master branch)

## 2.0.0-rc.1

* Upgrade Indexer Framework to be based on [nearcore 2.0.0-rc.1](https://github.com/near/nearcore/releases/tag/2.0.0-rc.1)
* Bump the minimal supported Rust version to `1.78.0` (following the `nearcore` master branch)

## 1.2.0

* Upgrade Indexer Framework to be based on [nearcore 1.40.0](https://github.com/near/nearcore/releases/tag/1.40.0)
* Fix s3 endpoint resolver
* Fix docker build

## 1.2.0-rc.1

* Upgrade Indexer Framework to be based on [nearcore 1.40.0-rc.1](https://github.com/near/nearcore/releases/tag/1.40.0-rc.1)
* Bump the minimal supported Rust version to `1.77.0` (following the `nearcore` master branch)
* Upgrade dependencies to the latest versions

## 1.1.1

* Upgrade Indexer Framework to be based on version 1.39.1 and commit [60c8ad8](https://github.com/near/nearcore/commit/60c8ad88e5615e173963ac73e318db9337a17134)

This commit is based on 1.39.1 release of `nearcore` and includes the improvement of local cache from the `rc.2` release since that commit hasn't get into the 1.39.x release.

The fix is expected to be included in 1.40.0 `nearcore` release.

## 1.1.0

* Upgrade Indexer Framework to be based on version 1.39.0 and commit [2b2c06e](https://github.com/near/nearcore/commit/2b2c06edb90400fb934ae08a7083250266bff780)

This commit is based on 1.39.0 release of `nearcore` and includes the improvement of local cache from the `rc.2` release since that commit hasn't get into the 1.39.x release.

## 1.0.0

* Upgrade Indexer Framework to be based on version 1.38.2 and commit [8c2085b](https://github.com/near/nearcore/commit/8c2085b84ce7643fada081bfc5986b09b7e575eb)

This commit is based on 1.38.2 release of `nearcore` and includes the improvement of local cache from the `rc.2` release since that commit hasn't get into the 1.38.x release.

## 1.0.0-rc.2

* Upgrade Indexer Framework to be based on commit [511414a](https://github.com/near/nearcore/commit/511414a5091c3bef5c447a5644ba903dc050b715)

This commit adds the local cache to improve the speed of indexing during the congestion of the network.

This release is expected to be compatible with `nearcore` 1.38.x but we don't expect that improvement to be cherry-picked to the 1.38.x release.

* Bump the minimal supported Rust version to `1.76.0` (following the `nearcore` master branch)

## 1.0.0-rc.1

After two years of successful work in production we realized that no major changes or improvements have been made to the codebase. The codebase is stable and mature enough to be considered as a stable release. Thus, we are happy to announce the first release candidate of the NEAR Lake 🎉

* Upgrade Indexer Framework to be based on [nearcore 1.38.0-rc.2](https://github.com/near/nearcore/releases/tag/1.38.0-rc.2)
* Observability improvements:
  * `near_lake_block_retry_count_total` Total number of retries for storing indexing blocks to S3
  * `near_lake_block_done_total` Total number of indexed blocks
  * `near_lake_build_info` similar to the existing near_build_info, but I cannot reuse that since it's private. It will expose the build info. Example:
  ```
  near_lake_build_info{build="1.37.1",release="0.1.29",rustc_version="1.75.0"}
  ```

## 0.1.29

* Upgrade Indexer Framework to be based on [nearcore 1.37.0](https://github.com/near/nearcore/releases/tag/1.37.0)

## 0.1.28

* Upgrade Indexer Framework to be based on [nearcore 1.36.5](https://github.com/near/nearcore/releases/tag/1.36.5)

## 0.1.27

* Upgrade Indexer Framework to be based on [nearcore 1.36.0-rc.1](https://github.com/near/nearcore/releases/tag/1.36.0-rc.1)

## 0.1.26

* Upgrade Indexer Framework to be based on [nearcore 1.35.0-rc.1](https://github.com/near/nearcore/releases/tag/1.35.0-rc.1)

## 0.1.25

* Fixed the dependency tree from `0.1.24` release that was broken due to the `nearcore` release.

## 0.1.24 (broken build see #74)

* Upgrade Indexer Framework to be based on [nearcore 1.34.0](https://github.com/near/nearcore/releases/tag/1.34.0)

## 0.1.23

* Upgrade Indexer Framework to be based on [nearcore 1.33.0-rc.1](https://github.com/near/nearcore/releases/tag/1.34.0-rc.1)
* Updated the minimal Rust version to `1.68.2`

## 0.1.22

* Upgrade Indexer Framework to be based on [nearcore 1.33.0-rc.1](https://github.com/near/nearcore/releases/tag/1.33.0-rc.1)
* Updated the minimal Rust version to `1.68.0`

## 0.1.21

* Upgrade Indexer Framework to be based on [nearcore 1.32.0-rc.1](https://github.com/near/nearcore/releases/tag/1.32.0-rc.1)
* Updated the minimal Rust version to `1.67.1`

## 0.1.21-rc.0

* Add support for meta transactions by upgrading near crates to [0.16](https://github.com/near/nearcore/commit/1a18003fe2f0873caac670bc0b86f8d59842b20a)

## 0.1.20

* Upgrade Indexer Framework to be based on [nearcore 1.31.1](https://github.com/near/nearcore/commit/825bc1b44b6d5080cc610d542e2b57f329d7aed9)

## 0.1.19

* Upgrade Indexer Framework to be based on [nearcore 1.31.0-rc.4](https://github.com/near/nearcore/commit/f709cdc89adfa0594df5ac20212e75402a1b862e)

### Heads-up

Some updates on the `nearcore` side affected the Indexer Framework:
- `near_client` calls require the usage of `near_o11y::WithSpanContextExt`. Thus we depend on `near-o11y` explicitly since this version
- `init_configs` function has been extended with a parameter `download_records_url: Option<&str>`. Thus `init` command of the Lake Indexer has been extended with the parameter `donwload_genesis_records_url`
- `IndexerConfig` requires new parameter `validate_genesis: bool` so the `run` command has been extended with the key `--validate-genesis`

## 0.1.18

* Upgrade Indexer Framework to be based on [nearcore 1.30.1](https://github.com/near/nearcore/commit/e2bf95c0737f7e80c70e77ae82b439342119148a)

## 0.1.17

* Upgrade Indexer Framework to be based on [nearcore 1.30.0](https://github.com/near/nearcore/commit/9b0275de057a01f87c259580f93e58f746da75aa)

## 0.1.16

* Upgrade Indexer Framework to be based on [nearcore 1.30.0-patch](https://github.com/near/nearcore/commit/267e36e39fb5bb29c1df23c73afbcaa750ce96b1)

## 0.1.15

* Upgrade Indexer Framework to be based on [nearcore 1.30.0-rc.2 release](https://github.com/near/nearcore/releases/tag/1.30.0-rc.2)

## 0.1.14

* Upgrade Indexer Framework to be based on [nearcore 1.29.0 release](https://github.com/near/nearcore/releases/tag/1.29.0)

## 0.1.13

* Upgrade `nearcore` to 1.29.0-rc.3

## 0.1.12

* Upgrade `nearcore` to 1.29.0-rc.2

## 0.1.11

* Upgrade `nearcore` to 1.29.0-rc.1

## 0.1.10

* Upgrade `nearcore` to 1.28.0

## 0.1.9

* Upgrade `nearcore` to 1.28.0-rc.1

## 0.1.8

* Upgrade `nearcore` to 1.27.0

## 0.1.7

* Upgrade `nearcore` to 1.27.0-rc.5

## 0.1.6

* Upgrade `nearcore` to 1.27.0-rc.4

## 0.1.5

* Upgrade `nearcore` to 1.27.0-rc.2

## 0.1.4

* Upgrade `nearcore` to 1.27.0-rc.1

## 0.1.3

* Upgrade `nearcore` to 1.26.0

## 0.1.2

* Fix: Calculation time to catch up with the network
* Upgrade `nearcore` to 1.26.0-rc.1

## 0.1.1

* Minor fix: avoid division by zero in stats printer function

## 0.1.0

* Make info logs easy to reason about (ref https://github.com/near/near-lake/issues/11)
* Optional `--endpoint` parameter to store the data to custom S3 compatible storage

## 0.1.0-rc.0

A first try in releasing the alpha version of NEAR Lake

* Runs NEAR Indexer and stores data to AWS S3
* Depends on `nearcore` commit that is not included in release yet https://github.com/near/nearcore/pull/6255
