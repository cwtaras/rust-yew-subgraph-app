use crate::components::data_table::DataTable;
use gloo_net::http::Request;
use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;
mod app;
mod components;
mod queries;
mod stores;
pub use app::*;

fn main() {
    yew::Renderer::<App>::new().render();
}
