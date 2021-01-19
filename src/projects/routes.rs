use bson::oid::ObjectId as mongoid;
use graphql_client::{GraphQLQuery, Response};
use handlebars::to_json;
use tide::Request;
use serde_json::value::Map;

use crate::State;
use crate::util::common::Tpl;
use crate::projects::models::Project;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_projects.graphql",
    response_derives = "Debug"
)]
struct AllProjects;
type ObjectId = String;

pub async fn project_index(_req: Request<State>) -> tide::Result {
    let project_index: Tpl = Tpl::new("project/index").await;

    // make data and render it
    let build_query = AllProjects::build_query(all_projects::Variables {});
    let query = serde_json::json!(build_query);

    let uri = "http://127.0.0.1:8080/v1";
    let resp_body: Response<all_projects::ResponseData> =
        surf::post(uri).body(query).recv_json().await.unwrap();

    let resp_data: all_projects::ResponseData = resp_body.data.expect("missing response data");

    let mut projects_data: Vec<Project> = vec![];
    for project in resp_data.all_projects {
        projects_data.push(Project {
            _id: mongoid::with_string(project.id.as_str()).unwrap(),
            user_id: mongoid::with_string(project.user_id.as_str()).unwrap(),
            subject: project.subject,
            website: project.website,
        })
    }

    let mut data = Map::new();
    data.insert("projects".to_string(), to_json(&projects_data));

    project_index.render(&data).await
}
