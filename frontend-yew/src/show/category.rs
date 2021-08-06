use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::{
    constant::CFG,
    common::{FetchState, fetch_gql_data},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/category.graphql"
)]
struct CategoryData;

async fn query_str(category_slug: String) -> String {
    let build_query = CategoryData::build_query(category_data::Variables {
        slug: category_slug,
    });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub category_slug: String,
}

pub struct Category {
    props: Props,
    data: FetchState<Value>,
    link: ComponentLink<Self>,
}

impl Component for Category {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, data: FetchState::NotFetching, link }
    }

    fn view(&self) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(category_data) => view_category(category_data),
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
                let props = self.props.clone();
                self.link.send_future(async {
                    match fetch_gql_data(&query_str(props.category_slug).await)
                        .await
                    {
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
        false
    }
}

fn view_category(category_data: &Value) -> Html {
    let category = &category_data["categoryBySlug"];
    let category_name = category["name"].as_str().unwrap();
    let document = yew::utils::document();
    document.set_title(&format!(
        "{} - {}",
        category_name,
        CFG.get("site.title").unwrap()
    ));

    let articles_vec = category["articles"].as_array().unwrap();
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
            <div class="s-card my6">
                <h2 class="mb6">
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
           { for articles }
        </>
    }
}