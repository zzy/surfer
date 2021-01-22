use graphql_client::{GraphQLQuery, Response};
use tide::Request;
use chrono::Local;

use crate::State;
use crate::util::common::{gql_uri, Tpl};

type ObjectId = String;
type DateTime = chrono::DateTime<Local>;

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
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

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

pub async fn article_new(_req: Request<State>) -> tide::Result {
    let articles_index: Tpl = Tpl::new("articles/new").await;

    let now = Local::now();

    // make data and render it
    let build_query = ArticleNew::build_query(article_new::Variables {
        user_id: "6007bf330013859f00f96fca".to_string(),
        subject: "测试项目19 what hahah the fuck".to_string(),
        content:
            "<span>抱歉，您正在使用的浏览器未被完全支持，我们强烈推荐您进行浏览器升级。</span>"
                .to_string(),
        created_at: now,
        updated_at: now,
    });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    articles_index.render(&resp_data).await
}
