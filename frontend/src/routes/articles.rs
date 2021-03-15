use std::collections::BTreeMap;
use tide::{Request, http::Method};
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
    query_path = "./graphql/article_new.graphql",
    response_derives = "Debug"
)]
struct ArticleNew;
use crate::models::articles::ArticleInfo;

pub async fn article_new(mut req: Request<State>) -> tide::Result {
    let mut article_new_tpl: Tpl = Tpl::new("articles/new").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    article_new_tpl.reg_head(&mut data).await;
    article_new_tpl.reg_sidebar(&mut data).await;

    article_new_tpl.reg_script_value_check().await;

    if req.method().eq(&Method::Post) {
        println!("1111111111");
        let article_info: ArticleInfo = req.body_form().await?;
        println!("{:?}", article_info.content);

        article_new_tpl.render(&data).await
    } else {
        println!("22222222222");
        article_new_tpl.render(&data).await
    }
    // let now = Local::now();

    // let build_query = ArticleNew::build_query(article_new::Variables {
    //     username: "test".to_string(),
    //     subject: "香洲半岛 2021 ... You sig---er tab or wi...ur session.".to_string(),
    //     content:
    //         "<span>抱歉，您正在使用的浏览器未被完全支持，我们强烈推荐您进行浏览器升级。</span>"
    //             .to_string(),
    //     created_at: now,
    //     updated_at: now,
    // });
    // let query = json!(build_query);

    // let resp_body: Response<serde_json::Value> =
    //     surf::post(&gql_uri().await).body(query).recv_json().await?;

    // let resp_data = resp_body.data.expect("missing response data");

    // article_new_tpl.render(&resp_data).await
}
