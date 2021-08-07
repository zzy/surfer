use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use yew::{
    prelude::*,
    virtual_dom::VNode,
    web_sys::{Request, RequestInit, RequestMode, Response},
};
use serde_json::{Value, from_str};

use crate::util::constant::CFG;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub async fn fetch_gql_data(query: &str) -> Result<Value, FetchError> {
    let mut req_opts = RequestInit::new();
    req_opts.method("POST");
    req_opts.body(Some(&JsValue::from_str(query)));
    req_opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&gql_uri().await, &req_opts)?;

    let window = yew::utils::window();
    let resp_value =
        JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_text = JsFuture::from(resp.text()?).await?;

    let data_str = resp_text.as_string().unwrap();
    let data_value: Value = from_str(&data_str).unwrap();

    Ok(data_value["data"].clone())
}

pub async fn gql_uri() -> String {
    let addr = CFG.get("gql.addr").unwrap();
    let path = CFG.get("gql.path").unwrap();

    format!("{}/{}", addr, path)
}

pub fn random_wish_node(wish_val: &Value) -> VNode {
    html! {
        <div class="ta-center mt16 mx64">
            <b>
                <a href={ format!("/{}", wish_val["user"]["username"].as_str().unwrap()) }
                    target="_blank">
                    { wish_val["user"]["nickname"].as_str().unwrap() }
                    { "@" }
                    { wish_val["user"]["blogName"].as_str().unwrap() }
                </a>
                { " shared the aphorism: " }
            </b>
            { wish_val["aphorism"].as_str().unwrap() }
            { " -- " }
            { wish_val["author"].as_str().unwrap() }
        </div>
    }
}

pub fn topic_tags_node(topic: &Value) -> VNode {
    let topic_quotes = topic["quotes"].as_i64().unwrap();
    let tag_size = if topic_quotes >= 100 {
        "s-tag__lg fw-bold"
    } else if topic_quotes < 100 && topic_quotes >= 60 {
        "s-tag__md fw-bold"
    } else if topic_quotes < 60 && topic_quotes >= 30 {
        "s-tag"
    } else if topic_quotes < 30 && topic_quotes >= 10 {
        "s-tag__sm"
    } else {
        "s-tag__xs"
    };

    html! {
        <a class={ classes!("s-tag", tag_size, "m8") }
            href={ topic["uri"].as_str().unwrap().to_string() } target="_blank">
            { topic["name"].as_str().unwrap() }
            { "（" }
            { topic_quotes }
            { "）" }
        </a>
    }
}
