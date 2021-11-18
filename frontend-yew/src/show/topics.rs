use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::{
    constant::CFG,
    common::{FetchState, fetch_gql_data},
};
use crate::components::nodes::{random_wish_node, topic_tag_node};

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
}

impl Component for Topics {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(topics_data) => view_topics(topics_data),
            FetchState::Failed(err) => html! { err },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetData);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetState(fetch_state) => {
                self.data = fetch_state;

                true
            }
            Msg::GetData => {
                ctx.link().send_future(async {
                    match fetch_gql_data(&query_str().await).await {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                ctx.link().send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(Msg::GetData);

        false
    }
}

fn view_topics(topics_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        "Topics",
        CFG.get("site.title").unwrap()
    ));

    let wish_val = &topics_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let topics_vec = topics_data["topics"].as_array().unwrap();
    let topics = topics_vec.iter().map(|topic| topic_tag_node(topic));

    html! {
        <>
            { random_wish }
            <div class="m24">
                { for topics }
            </div>
        </>
    }
}
