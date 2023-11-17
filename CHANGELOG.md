# Changelog

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
