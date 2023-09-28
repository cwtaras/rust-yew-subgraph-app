type BigDecimal = String;
use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery, Copy, Clone, Debug)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/queries/indexers.graphql",
    response_derives = "Debug,Clone,Eq,PartialEq,Default"
)]
pub struct IndexersQuery;

#[derive(GraphQLQuery, Copy, Clone, Debug)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/queries/latencies.graphql",
    response_derives = "Debug,Clone,Eq,PartialEq,Default"
)]
pub struct LatencyQuery;

#[derive(GraphQLQuery, Copy, Clone, Debug)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/queries/indexer_latencies.graphql",
    response_derives = "Debug,Clone,Eq,PartialEq,Default"
)]
pub struct IndexerLatenciesQuery;
