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
    data: FetchState<Value>,
}

impl Component for Category {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(category_data) => view_category(category_data),
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
                    match fetch_gql_data(&query_str(props.category_slug).await)
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

fn view_category(category_data: &Value) -> Html {
    let wish_val = &category_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let category = &category_data["categoryBySlug"];
    let category_name = category["name"].as_str().unwrap();
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        category_name,
        CFG.get("site.title").unwrap()
    ));

    let articles_vec = category["articles"].as_array().unwrap();
    let articles =
        articles_vec.iter().map(|article| article_card_node(article));

    html! {
        <>
            { random_wish }
            <div class="m24 mb8 p8 fs-subheading bg-blue-100 bg-confetti-animated">
                <b class="fc-danger">{ category_name }</b>
                { " 类目中，文章共 " }
                <b class="fc-danger">{ category["quotes"].as_i64().unwrap() }</b>
                { " 篇：" }
            </div>
            { for articles }
        </>
    }
}
