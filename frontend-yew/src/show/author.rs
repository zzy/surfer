use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::{
    constant::CFG,
    common::{FetchState, fetch_gql_data, random_wish_node},
};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/author.graphql"
)]
struct AuthorData;

async fn query_str(username: String) -> String {
    let build_query =
        AuthorData::build_query(author_data::Variables { username: username });
    let query = json!(build_query);

    query.to_string()
}

pub enum Msg {
    SetState(FetchState<Value>),
    GetData,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub username: String,
}

pub struct Author {
    props: Props,
    data: FetchState<Value>,
    link: ComponentLink<Self>,
}

impl Component for Author {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, data: FetchState::NotFetching, link }
    }

    fn view(&self) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(author_data) => view_author(author_data),
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
                    match fetch_gql_data(&query_str(props.username).await).await
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

fn view_author(author_data: &Value) -> Html {
    let wish_val = &author_data["randomWish"];
    let random_wish = random_wish_node(wish_val);

    let user = &author_data["userByUsername"];
    let username = user["username"].as_str().unwrap();
    let nickname = user["nickname"].as_str().unwrap();
    let blog_name = user["blogName"].as_str().unwrap();
    let document = yew::utils::document();
    document.set_title(&format!(
        "{} - {} - {} - {}",
        username,
        nickname,
        blog_name,
        CFG.get("site.title").unwrap()
    ));

    let articles_vec = user["articles"].as_array().unwrap();
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
                    <a href={ format!("/{}", username) }
                        target="_blank">
                        { nickname }
                        { "@" }
                        { blog_name }
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
            <div class="m24 mb8 p8 fs-subheading bg-blue-100 bg-confetti-animated">
                <b class="fc-warning">
                    { nickname }
                    { "@" }
                    { blog_name }
                </b>
                { " 文章分享列表：" }
            </div>
            { for articles }
        </>
    }
}
