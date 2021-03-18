use std::collections::BTreeMap;
use tide::{Request, Response, Redirect, http::Method};
use graphql_client::{GraphQLQuery, Response as GqlResponse};
use chrono::Local;
use serde_json::json;

use crate::State;
use crate::util::common::{gql_uri, Tpl};

type ObjectId = String;
type DateTime = chrono::DateTime<Local>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article_index.graphql"
)]
struct ArticleIndexData;

pub async fn article_index(req: Request<State>) -> tide::Result {
    let username = req.param("username").unwrap();
    let slug = req.param("slug").unwrap();

    // make data and render it
    let build_query =
        ArticleIndexData::build_query(article_index_data::Variables {
            username: username.to_string(),
            slug: slug.to_string(),
        });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();
    let resp_data = resp_body.data.expect("missing response data");

    let mut article_index_tpl: Tpl = Tpl::new("articles/index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    let categories = resp_data["categoriesByUsername"].clone();
    data.insert("categories", categories);

    let user = resp_data["articleBySlug"]["user"].clone();
    data.insert("user", user);

    let article = resp_data["articleBySlug"].clone();
    data.insert("article", article);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let topics = resp_data["topicsByUsername"].clone();
    data.insert("topics", topics);

    let articles = resp_data["articlesByUsername"].clone();
    data.insert("articles", articles);

    article_index_tpl.reg_head(&mut data).await;
    article_index_tpl.reg_header(&mut data).await;
    article_index_tpl.reg_nav(&mut data).await;
    article_index_tpl.reg_topic(&mut data).await;
    article_index_tpl.reg_elsewhere(&mut data).await;
    article_index_tpl.reg_footer(&mut data).await;

    article_index_tpl.reg_script_value_check().await;
    article_index_tpl.reg_script_website_svg().await;
    article_index_tpl.reg_script_sci_format().await;

    article_index_tpl.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/articles_list.graphql",
    response_derives = "Debug"
)]
struct ArticlesList;

pub async fn articles_list(_req: Request<State>) -> tide::Result {
    let articles_list_tpl: Tpl = Tpl::new("articles/list").await;

    // make data and render it
    let build_query = ArticlesList::build_query(articles_list::Variables {});
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await?;

    let resp_data = resp_body.data.expect("missing response data");

    articles_list_tpl.render(&resp_data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_dashboard.graphql",
    response_derives = "Debug"
)]
struct UserDashboardData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article_new.graphql",
    response_derives = "Debug"
)]
struct ArticleNewData;
use crate::models::articles::ArticleInfo;

pub async fn article_new(mut req: Request<State>) -> tide::Result {
    let mut username = String::new();
    if let Some(cookie) = req.cookie("username") {
        username.push_str(cookie.value());
    } else {
        username.push_str("-");
    }

    let mut sign_in = false;
    if "".ne(username.trim()) && "-".ne(username.trim()) {
        sign_in = true;
    }

    if sign_in {
        if req.method().eq(&Method::Post) {
            let article_info: ArticleInfo = req.body_form().await?;

            let now = Local::now();

            let build_query =
                ArticleNewData::build_query(article_new_data::Variables {
                    user_id: article_info.user_id,
                    subject: article_info.subject,
                    category_id: article_info.category_id,
                    summary: article_info.summary,
                    content: article_info.content,
                    created_at: now,
                    updated_at: now,
                });
            let query = json!(build_query);

            let resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await).body(query).recv_json().await?;
            let resp_data = resp_body.data.expect("missing response data");

            let article_uri =
                resp_data["articleNew"]["uri"].as_str().unwrap_or("/");
            let resp: Response =
                Redirect::new(format!("{}", article_uri)).into();

            Ok(resp.into())
        } else {
            let mut article_new_tpl: Tpl = Tpl::new("articles/new").await;
            let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

            article_new_tpl.reg_head(&mut data).await;
            article_new_tpl.reg_header(&mut data).await;
            article_new_tpl.reg_nav(&mut data).await;
            article_new_tpl.reg_sidebar(&mut data).await;
            article_new_tpl.reg_footer(&mut data).await;

            article_new_tpl.reg_script_value_check().await;
            article_new_tpl.reg_script_website_svg().await;

            let build_query = UserDashboardData::build_query(
                user_dashboard_data::Variables {
                    sign_in: sign_in,
                    username: username,
                },
            );
            let query = json!(build_query);

            let resp_body: GqlResponse<serde_json::Value> =
                surf::post(&gql_uri().await).body(query).recv_json().await?;
            let resp_data = resp_body.data.expect("missing response data");

            let user = resp_data["userByUsername"].clone();
            data.insert("user", user);

            let categories = resp_data["categories"].clone();
            data.insert("categories", categories);

            article_new_tpl.render(&data).await
        }
    } else {
        let resp: Response = Redirect::new("/sign-in").into();

        Ok(resp.into())
    }
}
