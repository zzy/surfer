use yew::prelude::*;
use graphql_client::GraphQLQuery;
use serde_json::{Value, json};

use crate::util::common::{FetchState, fetch_gql_data};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article.graphql"
)]
struct ArticleData;

async fn query_str(username: String, article_slug: String) -> String {
    let build_query = ArticleData::build_query(article_data::Variables {
        username: username,
        slug: article_slug,
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
    pub username: String,
    pub article_slug: String,
}

pub struct Article {
    props: Props,
    data: FetchState<Value>,
    link: ComponentLink<Self>,
}

impl Component for Article {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, data: FetchState::NotFetching, link }
    }

    fn view(&self) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(article_data) => view_article(article_data),
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
                    match fetch_gql_data(
                        &query_str(props.username, props.article_slug).await,
                    )
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

fn view_article(article_data: &Value) -> Html {
    let wish_val = &article_data["randomWish"];
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

    let article = &article_data["articleBySlug"];

    let article_topics_vec = article["topics"].as_array().unwrap();
    let article_topics = article_topics_vec.iter().map(|topic| {
        html! {
            <a class="s-badge s-badge__sm ml4 mb2"
                href={ topic["uri"].as_str().unwrap().to_string() } target="_blank">
                { topic["name"].as_str().unwrap() }
            </a>
        }
    });

    let content_html = article["contentHtml"].as_str().unwrap();
    let content_html_section =
        yew::utils::document().create_element("section").unwrap();
    content_html_section.set_class_name("fs-body2 mt24");
    content_html_section.set_inner_html(content_html);
    let content_html_node = Html::VRef(content_html_section.into());

    html! {
        <>
            { random_wish }
            <article class="s-card mx24 my12">
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
                    <b class="mr2">{ "Topics:" }</b>
                    { for article_topics }
                </p>
                <p class="fs-body1 my6 bg-gold-lighter">
                    <b class="mr2">{ "Summary:" }</b>
                    { article["summary"].as_str().unwrap() }
                </p>
                <link href="/css/night-owl.min.css" rel="stylesheet" />
                { content_html_node }
                <script src="/js/hl.js?132689068675031052"></script>
            </article>
        </>
    }
}
