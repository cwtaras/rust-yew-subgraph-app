use crate::queries::queries::{indexer_latencies_query, IndexerLatenciesQuery};
use crate::stores::store::IndexerLatencies;
use futures::select;
use futures::{FutureExt, TryFutureExt};
use graphql_client::reqwest::post_graphql;
use std::future::Future;
use thiserror::Error;
use yew::prelude::*;
use yewdux::prelude::use_store;

const SUBGRAPH_URL: &str =
    "https://api.thegraph.com/subgraphs/name/graphprotocol/gateway-mips-qos-oracle";

fn _log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s))
}

#[function_component(IndexersTable)]
pub fn indexers_table() -> Html {
    let counter: UseStateHandle<i64> = use_state(|| 20);
    let fetching = use_state(|| false);

    let onclick20 = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(20))
    };

    let onclick50 = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(50))
    };
    let onclick100 = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(100))
    };
    let (store, dispatch) = use_store::<IndexerLatencies>();
    let mdispatch = dispatch.clone();
    let mcount = *counter.clone();
    let mfetching = fetching.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                mfetching.set(true);
                let client = reqwest::Client::new();
                let variables = indexer_latencies_query::Variables {
                    latency_first: mcount,
                };
                let response =
                    post_graphql::<IndexerLatenciesQuery, _>(&client, SUBGRAPH_URL, variables)
                        .await
                        .unwrap()
                        .data
                        .unwrap();
                mdispatch.set(IndexerLatencies { data: response });
                mfetching.set(false);
            });
        },
        mcount,
    );

    let table_body = store
        .data
        .indexers
        .iter()
        .map(|indexer| {
            let data = indexer.daily_data_points.clone();
            let mut day_count: f64 = 0.0;
            let mut avg_ms: f64 = 0.0;
            let mut avg_block: f64 = 0.0;
            let mut query_count: f64 = 0.0;
            let mut total_query_fees: f64 = 0.0;
            for day in data.iter() {
                day_count += 1.0;
                avg_ms += day.avg_indexer_latency_ms.as_str().parse::<f64>().unwrap();
                avg_block += day
                    .avg_indexer_blocks_behind
                    .as_str()
                    .parse::<f64>()
                    .unwrap();
                query_count += day.query_count.as_str().parse::<f64>().unwrap();
                total_query_fees += day.total_query_fees.as_str().parse::<f64>().unwrap();
            }
            html! {
                <tr>
                    <td>{indexer.id.clone()}</td>
                    <td>{(avg_ms/day_count).ceil()}{" ms"}</td>
                    <td>{(avg_block/day_count).ceil()}</td>
                    <td>{(query_count/day_count).ceil()}</td>
                    <td>{total_query_fees/day_count}</td>
                    <td>{"arbitrum-one"}</td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div>
        <ybc::Title classes={classes!("has-text-black")} size={ybc::HeaderSize::Is5}>{"Last "}{*counter.clone()}</ybc::Title>
       <div >
       <ybc::Button onclick={onclick20}>{ "20" }</ybc::Button>
       <ybc::Button onclick={onclick50}>{ "50" }</ybc::Button>
       <ybc::Button onclick={onclick100}>{ "100" }</ybc::Button>

       if *fetching {
        <progress class="progress is-small is-primary block" max="100">{"15%"}</progress>
       }
       </div>

        <ybc::Table hoverable={true} fullwidth={true}>
        <thead>
            <tr>
                <th>{"Indexer ID"}</th>
                <th>{"Avg. Indexer Latency"}</th>
                <th>{"Avg. Indexer Block Behind"}</th>
                <th>{"Avg. Query Count"}</th>
                <th>{"Avg. Query Fees"}</th>
                <th>{"Chain ID"}</th>
            </tr>
        </thead>
        <tbody>
        {table_body}
        </tbody>
        </ybc::Table>
    </div>
    }
}
