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
    query_path = "./graphql/home.graphql"
)]
struct HomeData;

async fn query_str() -> String {
    let build_query = HomeData::build_query(home_data::Variables {});
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

pub struct Home {
    data: FetchState<Value>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(home_data) => view_home(home_data),
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

fn view_home(home_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        "Home",
        CFG.get("site.title").unwrap()
    ));

    let wish_val = &home_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let top_articles_vec = home_data["topArticles"].as_array().unwrap();
    let top_articles = top_articles_vec.iter().map(|top_article| {
        let article_topics_vec = top_article["topics"].as_array().unwrap();
        let article_topics = article_topics_vec.iter().map(|topic| {
            html! {
                <a class="s-badge s-badge__sm ml4 mb2"
                    href={ topic["uri"].as_str().unwrap().to_string() } target="_blank">
                    { topic["name"].as_str().unwrap() }
                </a>
            }
        });

        html! {
            <div class="s-card flex--item fl-equal wmn3 m6">
                <h2 class="mb6">
                    <a class="s-tag mr6"
                        href={ top_article["category"]["uri"].as_str().unwrap().to_string() }
                        target="_blank">
                        { top_article["category"]["name"].as_str().unwrap() }
                    </a>
                    <a href={ top_article["uri"].as_str().unwrap().to_string() } target="_blank">
                        { top_article["subject"].as_str().unwrap() }
                    </a>
                </h2>
                <p class="fs-caption my6">
                    { top_article["updatedAt"].as_str().unwrap() }
                    { " by " }
                    <a href={ format!("/{}", top_article["user"]["username"].as_str().unwrap()) }
                        target="_blank">
                        { top_article["user"]["nickname"].as_str().unwrap() }
                        { "@" }
                        { top_article["user"]["blogName"].as_str().unwrap() }
                    </a>
                </p>
                <p class="my6">
                    <b>{ "Topics:" }</b>
                    { for article_topics }
                </p>
                <p class="fs-body1 v-truncate3 mt6">{ top_article["summary"].as_str().unwrap() }</p>
            </div>
        }
    });

    let recommended_articles_vec =
        home_data["recommendedArticles"].as_array().unwrap();
    let recommended_articles = recommended_articles_vec
        .iter()
        .map(|recommended_article| article_card_node(recommended_article));

    html! {
        <>
            { random_wish }
            <div class="d-flex gsx fw-wrap m16">
                { for top_articles }
            </div>
            { for recommended_articles }
        </>
    }
}
