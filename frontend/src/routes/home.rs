use std::collections::BTreeMap;
use tide::Request;
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;

use crate::State;
use crate::util::common::{gql_uri, Tpl};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/index.graphql"
)]
struct IndexData;

pub async fn index(_req: Request<State>) -> tide::Result {
    // make data and render it
    let build_query = IndexData::build_query(index_data::Variables {
        username: "-".to_string(),
    });
    let query = json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await?;
    let resp_data = resp_body.data.expect("missing response data");

    let mut index: Tpl = Tpl::new("index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    let categories = resp_data["categories"].clone();
    data.insert("categories", categories);

    let top_articles = resp_data["topArticles"].clone();
    data.insert("top_articles", top_articles);

    let recommended_articles = resp_data["recommendedArticles"].clone();
    data.insert("recommended_articles", recommended_articles);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let articles = resp_data["articles"].clone();
    data.insert("articles", articles);

    let topics = resp_data["topics"].clone();
    data.insert("topics", topics);

    index.reg_head(&mut data).await;
    index.reg_header(&mut data).await;
    index.reg_nav(&mut data).await;
    index.reg_introduction(&mut data).await;
    index.reg_topic(&mut data).await;
    index.reg_elsewhere(&mut data).await;
    index.reg_pagination(&mut data).await;
    index.reg_footer(&mut data).await;

    index.reg_script_value_check().await;
    index.reg_script_website_svg().await;
    index.reg_script_sci_format().await;
    index.reg_script_str_trc().await;

    index.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_index.graphql"
)]
struct UserIndexData;

pub async fn user_index(req: Request<State>) -> tide::Result {
    let username = req.param("username").unwrap();

    // make data and render it
    let build_query = UserIndexData::build_query(user_index_data::Variables {
        username: username.to_string(),
    });
    let query = json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();
    let resp_data = resp_body.data.expect("missing response data");

    let mut user_index_tpl: Tpl = Tpl::new("users/index").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    let user = resp_data["userByUsername"].clone();
    data.insert("user", user);

    let categories = resp_data["categoriesByUsername"].clone();
    data.insert("categories", categories);

    let top_articles = resp_data["topArticles"].clone();
    data.insert("top_articles", top_articles);

    let recommended_articles = resp_data["recommendedArticles"].clone();
    data.insert("recommended_articles", recommended_articles);

    let wish = resp_data["randomWish"].clone();
    data.insert("wish", wish);

    let articles = resp_data["articlesByUsername"].clone();
    data.insert("articles", articles);

    let topics = resp_data["topicsByUsername"].clone();
    data.insert("topics", topics);

    user_index_tpl.reg_head(&mut data).await;
    user_index_tpl.reg_header(&mut data).await;
    user_index_tpl.reg_nav(&mut data).await;
    user_index_tpl.reg_introduction(&mut data).await;
    user_index_tpl.reg_topic(&mut data).await;
    user_index_tpl.reg_elsewhere(&mut data).await;
    user_index_tpl.reg_pagination(&mut data).await;
    user_index_tpl.reg_footer(&mut data).await;

    user_index_tpl.reg_script_value_check().await;
    user_index_tpl.reg_script_website_svg().await;
    user_index_tpl.reg_script_sci_format().await;
    user_index_tpl.reg_script_str_trc().await;

    user_index_tpl.render(&data).await
}

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

    let resp_body: Response<serde_json::Value> =
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
