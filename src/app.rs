use crate::components::data_table::DataTable;
use crate::components::indexers_table::IndexersTable;
use gloo_net::http::Request;
use serde::Deserialize;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ybc::TileCtx::{Ancestor, Child, Parent};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <ybc::Navbar
            classes={classes!("is-primary")}
            padded=true
            navbrand={html!{
                <ybc::NavbarItem>
                    <ybc::Title classes={classes!("has-text-white")} size={ybc::HeaderSize::Is4}>{"Dashboard"}</ybc::Title>
                </ybc::NavbarItem>
            }}
            navstart={html!{}}
            navend={html!{
                <>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes={classes!("is-white", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://thegraph.com/">
                        {"The Graph"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor classes={classes!("is-white", "is-outlined")} rel={String::from("noopener noreferrer")} target={String::from("_blank")} href="https://graphops.xyz/">
                        {"GraphOps"}
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>

                </>
            }}
            />


        <IndexersTable />
        //<DataTable />
        </div>

    }
}
