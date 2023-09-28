use crate::queries::queries::{
    indexer_latencies_query, indexers_query, latency_query, IndexerLatenciesQuery, IndexersQuery,
    LatencyQuery,
};
use crate::stores::store::{IndexerLatency, IndexerStore, LatencyStore};
use futures::select;
use futures::{FutureExt, TryFutureExt};
use graphql_client::reqwest::post_graphql;
use std::future::Future;
use thiserror::Error;
use yew::prelude::*;
use yewdux::prelude::use_store;
use yewdux::{dispatch, store};

const SUBGRAPH_URL: &str =
    "https://api.thegraph.com/subgraphs/name/graphprotocol/gateway-mips-qos-oracle";

fn _log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s))
}

async fn fetch_indexers() {}

#[function_component(DataTable)]
pub fn data_table() -> Html {
    let (lstore, ldispatch) = use_store::<LatencyStore>();
    let latency_dispatch = ldispatch.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let client = reqwest::Client::new();
        let indexer_variable = indexers_query::Variables {};
        let response = post_graphql::<IndexersQuery, _>(&client, SUBGRAPH_URL, indexer_variable)
            .await
            .unwrap()
            .data
            .unwrap()
            .indexers;
        let mut state: Vec<IndexerLatency> = [].to_vec();
        for indexer in response.iter() {
            let latency_variables = latency_query::Variables {
                indexer: indexer.id.clone(),
            };
            let lat_response =
                post_graphql::<LatencyQuery, _>(&client, SUBGRAPH_URL, latency_variables)
                    .await
                    .unwrap()
                    .data
                    .unwrap();

            let ind_lat: IndexerLatency = IndexerLatency {
                indexer: indexer.id.clone(),
                latency: lat_response,
            };
            state.push(ind_lat);
        }
        latency_dispatch.set(LatencyStore { latencies: state });
    });

    // let (lstore, ldispatch) = use_store::<LatencyStore>();
    // let indexer_store = store.clone();
    // let latency_dispatch = ldispatch.clone();
    // let latency_store = lstore.clone();
    // wasm_bindgen_futures::spawn_local(async move {
    //     for indexer in indexer_store.indexers.indexers.iter() {
    //         let client = reqwest::Client::new();
    //         let latency_variables = latency_query::Variables {
    //             indexer: indexer.id.clone(),
    //         };
    //         let response =
    //             post_graphql::<LatencyQuery, _>(&client, SUBGRAPH_URL, latency_variables)
    //                 .await
    //                 .unwrap()
    //                 .data
    //                 .unwrap();
    //         let ind_lat: IndexerLatency = IndexerLatency {
    //             indexer: indexer.id.clone(),
    //             latency: response,
    //         };
    //         _log("inside");
    //         latency_dispatch.reduce(|state| {
    //             let mut state = state.as_ref().clone();
    //             state.latencies.push(ind_lat);
    //             state.into()
    //         });
    //     }

    //     indexer_store.indexers.indexers.iter().map(|indexer| async {
    //         let client = reqwest::Client::new();
    //         let latency_variables = latency_query::Variables {
    //             indexer: indexer.id.clone(),
    //         };
    //         let response =
    //             post_graphql::<LatencyQuery, _>(&client, SUBGRAPH_URL, latency_variables)
    //                 .await
    //                 .unwrap()
    //                 .data
    //                 .unwrap();
    //         let ind_lat: IndexerLatency = IndexerLatency {
    //             indexer: indexer.id.clone(),
    //             latency: response,
    //         };
    //         _log("inside");
    //         latency_dispatch.reduce(|state| {
    //             let mut state = state.as_ref().clone();
    //             state.latencies.push(ind_lat);
    //             state.into()
    //         });
    //     });
    // });

    let latencies = lstore
        .latencies
        .iter()
        .map(|indlat| {
            let data = indlat.latency.indexer_daily_data_points.clone();
            let mut avg_ms: f64 = 0.0;
            let mut avg_block: f64 = 0.0;
            let mut dayNumber = 0.0;
            for day in data.iter() {
                avg_ms += day.avg_indexer_latency_ms.as_str().parse::<f64>().unwrap();
                avg_block += day
                    .avg_indexer_blocks_behind
                    .as_str()
                    .parse::<f64>()
                    .unwrap();
                if (indlat
                    .indexer
                    .eq("0x2121bc6437100fc21d19a9eea30898419e020afa"))
                {
                    _log(day.avg_indexer_latency_ms.as_str());
                }
            }
            html! {
                <tr>
                    <td>{indlat.indexer.clone()}</td>
                    <td>{(avg_ms/20.0).ceil()}{" ms"}</td>
                    <td>{(avg_block/20.0).ceil()}</td>
                    <td>{"arbitrum-one"}</td>
                </tr>
            }
        })
        .collect::<Html>();

    _log("herehere");
    html! {
        <div>
            <ybc::Table hoverable={true} fullwidth={true}>
            <thead>
                <tr>
                    <th>{"Indexer ID"}</th>
                    <th>{"Avg. Indexer Latency (last 20)"}</th>
                    <th>{"Avg. Indexer Block Behind (last 20)"}</th>
                    <th>{"Chain ID"}</th>
                </tr>
            </thead>
            <tbody>
            {latencies}
            </tbody>
            </ybc::Table>

            <p></p>


        </div>
    }
}
