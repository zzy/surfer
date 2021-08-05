use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::common::{FetchState, fetch_gql_data};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/articles.graphql"
)]
struct ArticlesData;

async fn query_str() -> String {
    let build_query = ArticlesData::build_query(articles_data::Variables {});
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

pub struct Articles {
    data: FetchState<Value>,
    link: ComponentLink<Self>,
}

impl Component for Articles {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { data: FetchState::NotFetching, link }
    }

    fn view(&self) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(articles_data) => view_articles(articles_data),
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

fn view_articles(articles_data: &Value) -> Html {
    let wish_val = &articles_data["randomWish"];
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

    let articles_vec = articles_data["articles"].as_array().unwrap();
    let articles = articles_vec.iter().map(|article| {
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
            <div class="s-card mx24 my12">
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
    });

    html! {
        <>
            { random_wish }
            { for articles }
        </>
    }
}
