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
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
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
    let user_new_tpl: Tpl = Tpl::new("users/new").await;

    let now = Local::now();

    // make data and render it
    let build_query = UserRegister::build_query(user_register::Variables {
        email: "test@budshome.com".to_string(),
        username: "haha".to_string(),
        cred: "budshome".to_string(),
        blog_name: "默默爸".to_string(),
        website: "https://github.com/zzy".to_string(),
        created_at: now,
        updated_at: now,
    });
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    user_new_tpl.render(&resp_data).await
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/user_by_username.graphql",
    response_derives = "Debug"
)]
struct UserByUsername;

pub async fn user_index(req: Request<State>) -> tide::Result {
    let user_new_tpl: Tpl = Tpl::new("users/index").await;

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

    user_new_tpl.render(&resp_data).await
}
