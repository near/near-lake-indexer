use near_o11y::metrics::*;
use once_cell::sync::Lazy;

pub static BLOCKS_DONE: Lazy<IntCounter> = Lazy::new(|| {
    try_create_int_counter(
        "near_lake_block_done_total",
        "Total number of indexed blocks",
    )
    .unwrap()
});

pub static RETRY_COUNT: Lazy<IntCounter> = Lazy::new(|| {
    try_create_int_counter(
        "near_lake_block_retry_count_total",
        "Total number of retries for storing indexing blocks to S3",
    )
    .unwrap()
});

// This metric is present in the near_o11y crate but it's not public
// so we can't use it directly. We have to redefine it here.
pub static NODE_BUILD_INFO: Lazy<IntCounterVec> = Lazy::new(|| {
    try_create_int_counter_vec(
        "near_lake_build_info",
        "Metric whose labels indicate node’s version; see \
             <https://www.robustperception.io/exposing-the-software-version-to-prometheus>.",
        &["release", "build", "rustc_version"],
    )
    .unwrap()
});
