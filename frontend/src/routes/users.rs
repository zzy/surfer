use tide::Request;
use std::collections::BTreeMap;
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

    let resp_body: GqlResponse<serde_json::Value> =
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

pub async fn user_dashboard(_req: Request<State>) -> tide::Result {
    let mut user_dashboard_tpl: Tpl = Tpl::new("users/dashboard").await;
    let mut data: BTreeMap<&str, serde_json::Value> = BTreeMap::new();

    user_dashboard_tpl.reg_head(&mut data).await;
    user_dashboard_tpl.reg_header(&mut data).await;
    user_dashboard_tpl.reg_nav(&mut data).await;
    user_dashboard_tpl.reg_sidebar(&mut data).await;

    user_dashboard_tpl.reg_script_value_check().await;
    user_dashboard_tpl.reg_script_website_svg().await;

    user_dashboard_tpl.render(&data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/users_list.graphql",
    response_derives = "Debug"
)]
struct UsersList;

pub async fn users_list(_req: Request<State>) -> tide::Result {
    let users_list_tpl: Tpl = Tpl::new("users/list").await;

    // make data and render it
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJlbWFpbCI6Im9rYTIyQGJ1ZHNob21lLmNvbSIsInVzZXJuYW1lIjoi5oiRMjJz6LCBMjRvazMyIiwiZXhwIjoxMDAwMDAwMDAwMH0.FUdYJeEL1eCfturVUoPYKaVG-m4e-Jl3YJviYg1b8O9hKw2rrH7HKZED0gDT4i5lKbI9VTfbI0Qu4Tt3apwpOw";
    let build_query = UsersList::build_query(users_list::Variables {
        token: token.to_string(),
    });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    users_list_tpl.render(&resp_data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_register.graphql",
    response_derives = "Debug"
)]
struct UserRegister;

pub async fn user_register(_req: Request<State>) -> tide::Result {
    let user_new_tpl: Tpl = Tpl::new("users/register").await;

    let now = Local::now();

    // make data and render it
    let build_query = UserRegister::build_query(user_register::Variables {
        email: "test3@budshome.com".to_string(),
        username: "test3".to_string(),
        nickname: "默默爸 TeSt 3".to_string(),
        cred: "test".to_string(),
        blog_name: "默默爸 TeSt 3".to_string(),
        website: "https://github.com/zzy".to_string(),
        created_at: now,
        updated_at: now,
    });
    let query = json!(build_query);

    let resp_body: GqlResponse<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    user_new_tpl.render(&resp_data).await
}
