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
}

impl Component for Articles {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { data: FetchState::NotFetching }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(articles_data) => view_articles(articles_data),
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

fn view_articles(articles_data: &Value) -> Html {
    let document = gloo_utils::document();
    document.set_title(&format!(
        "{} - {}",
        "Articles",
        CFG.get("site.title").unwrap()
    ));

    let wish_val = &articles_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let articles_vec = articles_data["articles"].as_array().unwrap();
    let articles =
        articles_vec.iter().map(|article| article_card_node(article));

    html! {
        <>
            { random_wish }
            { for articles }
        </>
    }
}
