use bson::oid::ObjectId as mongoid;
use graphql_client::{GraphQLQuery, Response};
use handlebars::to_json;
use tide::Request;
use serde_json::value::Map;

use crate::State;
use crate::util::common::Tpl;
use crate::users::models::User;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_users.graphql",
    response_derives = "Debug"
)]
struct AllUsers;
type ObjectId = String;

pub async fn user_index(_req: Request<State>) -> tide::Result {
    let project_index: Tpl = Tpl::new("user/index").await;

    // make data and render it
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6InNpZ25pbmdfa2V5In0.eyJlbWFpbCI6Im9rYTIyQGJ1ZHNob21lLmNvbSIsInVzZXJuYW1lIjoi5oiRMjJz6LCBMjRvazMyIiwiZXhwIjoxMDAwMDAwMDAwMH0.mw2OP6A6uW2W0hEHNk3C5Mq8QoAwT-xfaUXZmP0I9qEsaeO26ORZgRIFL3t1C0JtdTNfYoIFiMbBrPRY5nBjKg";
    let build_query = AllUsers::build_query(all_users::Variables { token: token.to_string() });
    let query = serde_json::json!(build_query);

    let uri = "http://127.0.0.1:8080/v1";
    let resp_body: Response<all_users::ResponseData> =
        surf::post(uri).body(query).recv_json().await.unwrap();

    let resp_data: all_users::ResponseData = resp_body.data.expect("missing response data");

    let mut users_data: Vec<User> = vec![];
    for user in resp_data.all_users {
        users_data.push(User {
            _id: mongoid::with_string(user.id.as_str()).unwrap(),
            email: user.email,
            username: user.username,
            cred: "".to_string(),
        })
    }

    let mut data = Map::new();
    data.insert("users".to_string(), to_json(&users_data));

    project_index.render(&data).await
}
