use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::common::{FetchState, fetch_gql_data};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topics.graphql"
)]
struct TopicsData;

async fn query_str() -> String {
    let build_query = TopicsData::build_query(topics_data::Variables {});
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

pub struct Topics {
    data: FetchState<Value>,
    link: ComponentLink<Self>,
}

impl Component for Topics {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { data: FetchState::NotFetching, link }
    }

    fn view(&self) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(topics_data) => view_topics(topics_data),
            FetchState::Failed(err) => html! { err },
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::GetData);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetState(fetch_state) => {
                self.data = fetch_state;

                true
            }
            Msg::GetData => {
                self.link.send_future(async {
                    match fetch_gql_data(&query_str().await).await {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                self.link.send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::GetData);

        false
    }
}

fn view_topics(topics_data: &Value) -> Html {
    let document = yew::utils::document();
    document.set_title(&format!("{} - {}", "Topics", document.title()));

    let wish_val = &topics_data["randomWish"];
    let random_wish = html! {
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
    };

    let topics_vec = topics_data["topics"].as_array().unwrap();
    let topics = topics_vec.iter().map(|topic| {
        let topic_quotes = topic["quotes"].as_i64().unwrap();
        let tag_size =
            if topic_quotes >= 100 {
                "s-tag__lg fw-bold"
            }
            else if topic_quotes < 100 && topic_quotes >= 60 {
                "s-tag__md fw-bold"
            }
            else if topic_quotes < 60 && topic_quotes >= 30 {
                "s-tag"
            }
            else if topic_quotes < 30 && topic_quotes >= 10 {
                "s-tag__sm"
            }
            else {
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
    });

    html! {
        <>
            { random_wish }
            <div class="m24">
                { for topics }
            </div>
        </>
    }
}
