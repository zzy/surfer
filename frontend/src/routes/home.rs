use std::collections::BTreeMap;
use tide::Request;
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;
use chrono::Local;

use crate::State;
use crate::util::common::{gql_uri, rhai_dir, tpl_dir, Tpl};

type ObjectId = String;
type DateTime = chrono::DateTime<Local>;

pub async fn index(_req: Request<State>) -> tide::Result {
    let mut index: Tpl = Tpl::new("index").await;
    index.reg.register_script_helper_file(
        "blog-name",
        format!("{}{}", rhai_dir().await, "blog-name.rhai"),
    )?;

    let data = ();

    index.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_by_username.graphql",
    response_derives = "Debug"
)]
struct UserByUsername;

pub async fn user_index(req: Request<State>) -> tide::Result {
    let mut user_tpl: Tpl = Tpl::new("users/index").await;
    user_tpl.reg.register_script_helper_file(
        "website-svg",
        format!("{}{}", rhai_dir().await, "website-svg.rhai"),
    )?;
    user_tpl.reg.register_script_helper_file(
        "blog-name",
        format!("{}{}", rhai_dir().await, "blog-name.rhai"),
    )?;

    let username = req.param("username").unwrap();

    // make data and render it
    let build_query =
        UserByUsername::build_query(user_by_username::Variables {
            username: username.to_string(),
        });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    let mut data: BTreeMap<&str, &serde_json::Value> = BTreeMap::new();
    let a = json!("很好哈");
    data.insert("data1", &a);
    data.insert("user", &resp_data["userByUsername"]);

    user_tpl.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/article_by_slug.graphql",
    response_derives = "Debug"
)]
struct ArticleBySlug;

pub async fn article_index(req: Request<State>) -> tide::Result {
    let mut article_tpl: Tpl = Tpl::new("articles/index").await;

    article_tpl.reg.register_template_file(
        "base",
        format!("{}{}", tpl_dir().await, "base.html"),
    )?;

    article_tpl.reg.register_script_helper_file(
        "website-svg",
        format!("{}{}", rhai_dir().await, "website-svg.rhai"),
    )?;
    article_tpl.reg.register_script_helper_file(
        "blog-name",
        format!("{}{}", rhai_dir().await, "blog-name.rhai"),
    )?;

    let username = req.param("username").unwrap();
    let slug = req.param("slug").unwrap();

    // make data and render it
    let build_query = ArticleBySlug::build_query(article_by_slug::Variables {
        username: username.to_string(),
        slug: slug.to_string(),
    });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");
    println!("{:?}", &resp_data["articleBySlug"]);

    let mut data: BTreeMap<&str, &serde_json::Value> = BTreeMap::new();
    let a = json!("很好哈");
    let b = json!("base".to_string());
    data.insert("parent", &b);
    data.insert("data1", &a);
    data.insert("article", &resp_data["articleBySlug"]);

    article_tpl.render(&data).await
}
