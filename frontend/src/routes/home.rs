use std::collections::BTreeMap;
use tide::Request;
use graphql_client::{GraphQLQuery, Response};
use serde_json::json;
use chrono::Local;

use crate::State;
use crate::util::common::{gql_uri, tpl_dir, Tpl};

type ObjectId = String;
type DateTime = chrono::DateTime<Local>;

pub async fn index(_req: Request<State>) -> tide::Result {
    let mut index: Tpl = Tpl::new("index").await;
    index.reg_script_blog_name().await;

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
    user_tpl.reg_script_website_svg().await;
    user_tpl.reg_script_blog_name().await;

    let username = req.param("username").unwrap();

    // make data and render it
    let build_query =
        UserByUsername::build_query(user_by_username::Variables {
            username: username.to_string(),
        });
    let query = json!(build_query);

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
    // article_tpl.reg.register_template_file(
    //     "base2",
    //     format!("{}{}", tpl_dir().await, "base2.html"),
    // )?;




    let username = req.param("username").unwrap();
    let slug = req.param("slug").unwrap();

    // make data and render it
    let build_query = ArticleBySlug::build_query(article_by_slug::Variables {
        username: username.to_string(),
        slug: slug.to_string(),
    });
    let query = json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");
    println!("{:?}", &resp_data);

    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();
    let a = json!("很好哈");
    let a2 = json!("不太好哈");
    let b = json!("base");
    // let b2 = json!("base2");

    data.insert("parent", b);
    // data.insert("base2", &b2);

    data.insert("data1", a);
    data.insert("data2", a2);

    data.insert("article", resp_data["articleBySlug"].clone());
    data.insert("user", resp_data["userByUsername"].clone());

    article_tpl.reg_script_website_svg().await;
    article_tpl.reg_script_blog_name().await;

    article_tpl.reg_head(&mut data).await;
    article_tpl.reg_header(&mut data).await;
    article_tpl.reg_nav(&mut data).await;
    article_tpl.reg_footer(&mut data).await;

    article_tpl.render(&data).await
}
