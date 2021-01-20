use graphql_client::{GraphQLQuery, Response};
use tide::Request;
use bson::oid::ObjectId;

use crate::State;
use crate::util::common::{gql_uri, Tpl};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_projects.graphql",
    response_derives = "Debug"
)]
struct AllProjects;

pub async fn project_index(_req: Request<State>) -> tide::Result {
    let project_index: Tpl = Tpl::new("project/index").await;

    // make data and render it
    let build_query = AllProjects::build_query(all_projects::Variables {});
    let query = serde_json::json!(build_query);

    let resp_body: Response<serde_json::Value> =
        surf::post(&gql_uri().await).body(query).recv_json().await.unwrap();

    let resp_data = resp_body.data.expect("missing response data");

    project_index.render(&resp_data).await
}
