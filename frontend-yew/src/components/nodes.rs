use yew::{prelude::*, virtual_dom::VNode};
use serde_json::Value;

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

pub fn article_card_node(article: &Value) -> VNode {
    let article_topics_vec = article["topics"].as_array().unwrap();
    let article_topics = article_topics_vec.iter().map(|topic| {
        html! {
            <a class="s-badge s-badge__sm ml4 mb2"
                href={ topic["uri"].as_str().unwrap().to_string() } target="_blank">
                { topic["name"].as_str().unwrap() }
            </a>
        }
    });

    html! {
        <div class="s-card mx24 my8">
            <h2 class="mb6">
                <a class="s-tag mr6"
                    href={ article["category"]["uri"].as_str().unwrap().to_string() }
                    target="_blank">
                    { article["category"]["name"].as_str().unwrap() }
                </a>
                <a href={ article["uri"].as_str().unwrap().to_string() } target="_blank">
                    { article["subject"].as_str().unwrap() }
                </a>
            </h2>
            <p class="fs-caption my6">
                { article["updatedAt"].as_str().unwrap() }
                { " by " }
                <a href={ format!("/{}", article["user"]["username"].as_str().unwrap()) }
                    target="_blank">
                    { article["user"]["nickname"].as_str().unwrap() }
                    { "@" }
                    { article["user"]["blogName"].as_str().unwrap() }
                </a>
            </p>
            <p class="my6">
                <b>{ "Topics:" }</b>
                { for article_topics }
            </p>
            <p class="fs-body1 v-truncate3 mt6">{ article["summary"].as_str().unwrap() }</p>
        </div>
    }
}

pub fn topic_tag_node(topic: &Value) -> VNode {
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

pub fn page_not_found() -> VNode {
    html! {
        <div class="ta-center mt16 mx64">
            <h1>
               { "无此页面" }
               <br/>
                { "Page not found" }
            </h1>
            <h3>
                { "似乎不存在此页面" }
                <br/>
                { "Page page does not seem to exist" }
            </h3>
        </div>
    }
}
