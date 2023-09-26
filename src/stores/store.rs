use crate::queries::queries::{indexers_query, latency_query};
use yewdux::prelude::*;

#[derive(Store, Eq, PartialEq, Clone, Default)]
pub struct IndexerStore {
    pub indexers: indexers_query::ResponseData,
}

#[derive(Eq, PartialEq, Clone, Default)]
pub struct IndexerLatency {
    pub indexer: String,
    pub latency: latency_query::ResponseData,
}

#[derive(Store, Eq, PartialEq, Clone, Default)]
pub struct LatencyStore {
    pub latencies: Vec<IndexerLatency>,
}
