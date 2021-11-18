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
    query_path = "./graphql/categories.graphql"
)]
struct CategoriesData;

async fn query_str() -> String {
    let build_query =
        CategoriesData::build_query(categories_data::Variables {});
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

pub struct Categories {
    data: FetchState<Value>,
}

impl Component for Categories {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(categories_data) => {
                view_categories(categories_data)
            }
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

fn view_categories(categories_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        "Categories",
        CFG.get("site.title").unwrap()
    ));

    let wish_val = &categories_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let categories_vec = categories_data["categories"].as_array().unwrap();
    let categories = categories_vec.iter().map(|category| {
        let topics_vec = category["topics"].as_array().unwrap();
        let topics = topics_vec.iter().map(|topic| topic_tag_node(topic));

        html! {
            <div class="ba bc-blue-100 m24">
                <span class="s-badge fs-body3 fw-bold">
                    <span class="s-award-bling s-award-bling__gold">
                        <a href={ category["uri"].as_str().unwrap().to_string() } target="_blank">
                            { category["name"].as_str().unwrap() }
                            { " - 共 " }
                            { category["quotes"].as_i64().unwrap() }
                            { " 篇" }
                        </a>
                    </span>
                </span>
                <div class="my6">
                    { for topics }
                </div>
            </div>
        }
    });

    html! {
        <>
            { random_wish }
            { for categories }
        </>
    }
}
