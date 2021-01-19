use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_projects.graphql",
    response_derives = "Debug"
)]
pub struct AllProjects;
type ObjectId = String;

#[async_std::main]
async fn main() -> surf::Result<()> {
    let uri = "http://127.0.0.1:8080/v1";

    let build_query = AllProjects::build_query(all_projects::Variables {});
    let query = serde_json::json!(build_query);
    println!("1-{:?}\n", &query);

    let resp_body: Response<all_projects::ResponseData> =
        surf::post(uri).body(query).recv_json().await.unwrap();
    println!("2-{:?}\n", &resp_body);

    let resp_data: all_projects::ResponseData = resp_body.data.expect("missing response data");
    println!("3-{:?}\n", resp_data);

    let projects_data = resp_data.all_projects;
    println!("4-{:?}\n", &projects_data);

    for projects in &projects_data {
        println!("{}", projects.id);
    }

    Ok(())
}
