use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::{
    constant::CFG,
    common::{FetchState, fetch_gql_data},
};
use crate::components::nodes::{random_wish_node, article_card_node};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/topic.graphql"
)]
struct TopicData;

async fn query_str(topic_slug: String) -> String {
    let build_query =
        TopicData::build_query(topic_data::Variables { slug: topic_slug });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub topic_slug: String,
}

pub struct Topic {
    data: FetchState<Value>,
}

impl Component for Topic {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(topic_data) => view_topic(topic_data),
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
                let props = ctx.props().clone();
                ctx.link().send_future(async {
                    match fetch_gql_data(&query_str(props.topic_slug).await)
                        .await
                    {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                ctx.link().send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }
}

fn view_topic(topic_data: &Value) -> Html {
    let wish_val = &topic_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let topic = &topic_data["topicBySlug"];
    let topic_name = topic["name"].as_str().unwrap();
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        topic_name,
        CFG.get("site.title").unwrap()
    ));

    let articles_vec = topic["articles"].as_array().unwrap();
    let articles =
        articles_vec.iter().map(|article| article_card_node(article));

    html! {
        <>
            { random_wish }
            <div class="m24 mb8 p8 fs-subheading bg-blue-100 bg-confetti-animated">
                <b class="fc-danger">{ topic_name }</b>
                { " 话题下，文章共 " }
                <b class="fc-danger">{ topic["quotes"].as_i64().unwrap() }</b>
                { " 篇：" }
            </div>
            { for articles }
        </>
    }
}
