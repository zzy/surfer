use std::fmt::Debug;

use yew::{format::Json, prelude::*};
use yew_services::fetch::{FetchService, FetchTask, Request, Response};

use graphql_client::GraphQLQuery;
use serde_json::{Value, from_str};

use crate::util::common::gql_uri;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/home.graphql",
    response_derives = "Debug"
)]
struct HomeData;

#[derive(Debug)]
pub enum Msg {
    FetchRequest,
    ReceiveResponse(Result<Value, anyhow::Error>),
}

#[derive(Debug)]
pub struct Home {
    fetch_task: Option<FetchTask>,
    data: Option<Value>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Home {
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }

    fn view_data(&self) -> Html {
        match self.data {
            Some(ref data) => {
                let wish_val = &data["randomWish"];
                let random_wish = html! {
                    <div class="ta-center mx32">
                        <b>
                            <a href=format!("/{}", wish_val["user"]["username"].as_str().unwrap()) target="_blank">
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

                let top_articles_vec = data["topArticles"].as_array().unwrap();
                let top_articles = top_articles_vec.iter().map(|top_article| {
                    let article_topics_vec = top_article["topics"].as_array().unwrap();
                    let article_topics = article_topics_vec.iter().map(|topic| {
                        html! {
                            <a class="s-badge s-badge__sm ml4 mb2" 
                                href={ topic["uri"].as_str().unwrap().to_owned() } target="_blank">
                                { topic["name"].as_str().unwrap() }
                            </a>
                        }
                    });

                    html! {
                        <div class="s-card flex--item fl-equal wmn3 m6">
                            <h2 class="mb6">
                                <a class="s-tag mr6" 
                                    href={ top_article["category"]["uri"].as_str().unwrap().to_owned() } target="_blank">
                                    { top_article["category"]["name"].as_str().unwrap() }
                                </a>
                                <a href={ top_article["uri"].as_str().unwrap().to_owned() } target="_blank">
                                    { top_article["subject"].as_str().unwrap() }
                                </a>
                            </h2>
                            <p class="fs-caption my6">
                                { top_article["updatedAt"].as_str().unwrap() }
                                { " by " }
                                <a href=format!("/{}", top_article["user"]["username"].as_str().unwrap()) target="_blank">
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

                let recommended_articles_vec = data["recommendedArticles"].as_array().unwrap();
                let recommended_articles = recommended_articles_vec.iter().map(|recommended_article| {
                    let article_topics_vec = recommended_article["topics"].as_array().unwrap();
                    let article_topics = article_topics_vec.iter().map(|topic| {
                        html! {
                            <a class="s-badge s-badge__sm ml4 mb2" 
                                href={ topic["uri"].as_str().unwrap().to_owned() } target="_blank">
                                { topic["name"].as_str().unwrap() }
                            </a>
                        }
                    });

                    html! {
                        <div class="s-card mx24 my6">
                            <h2 class="mb6">
                                <a class="s-tag mr6" 
                                    href={ recommended_article["category"]["uri"].as_str().unwrap().to_owned() } target="_blank">
                                    { recommended_article["category"]["name"].as_str().unwrap() }
                                </a>
                                <a href={ recommended_article["uri"].as_str().unwrap().to_owned() } target="_blank">
                                    { recommended_article["subject"].as_str().unwrap() }
                                </a>
                            </h2>
                            <p class="fs-caption my6">
                                { recommended_article["updatedAt"].as_str().unwrap() }
                                { " by " }
                                <a href=format!("/{}", recommended_article["user"]["username"].as_str().unwrap()) target="_blank">
                                    { recommended_article["user"]["nickname"].as_str().unwrap() }
                                    { "@" }
                                    { recommended_article["user"]["blogName"].as_str().unwrap() }
                                </a>
                            </p>
                            <p class="my6">
                                <b>{ "Topics:" }</b>
                                { for article_topics }
                            </p>
                            <p class="fs-body1 v-truncate3 mt6">{ recommended_article["summary"].as_str().unwrap() }</p>
                        </div>
                    }
                });

                html! {
                    <>
                        { random_wish }
                        <div class="d-flex gs32 fw-wrap p32 sm:p24">
                            { for top_articles }
                        </div>
                        { for recommended_articles }
                    </>
                }
            }
            None => {
                html! {
                     <p>
                        { "No data." }
                     </p>
                }
            }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { fetch_task: None, data: None, link, error: None }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_data() }
                { self.view_error() }
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.link.send_message(Msg::FetchRequest);
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchRequest => {
                let build_query =
                    HomeData::build_query(home_data::Variables {});
                let query = Json(&build_query);

                let request = Request::post(&gql_uri())
                    .body(query)
                    .expect("Could not build request.");

                let callback = self.link.callback(
                    |response: Response<Result<String, anyhow::Error>>| {
                        let resp_body = response.into_body();
                        let resp_str = resp_body.as_ref().unwrap();

                        let resp_value: Value = from_str(&resp_str).unwrap();
                        let resp_data = resp_value["data"].to_owned();

                        Msg::ReceiveResponse(Ok(resp_data))
                    },
                );

                let task = FetchService::fetch(request, callback)
                    .expect("Failed to start request");
                self.fetch_task = Some(task);

                true
            }
            Msg::ReceiveResponse(resp_data) => {
                match resp_data {
                    Ok(data) => {
                        self.data = Some(data);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.link.send_message(Msg::FetchRequest);

        false
    }
}
