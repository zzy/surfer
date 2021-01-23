pub mod queries;
pub mod mutations;

use tide::http::mime;
use tide::{Request, Response, StatusCode};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Schema, EmptySubscription};

use crate::State;

use crate::util::constant::CFG;
use crate::dbs::mongo;

use crate::gql::queries::QueryRoot;
use crate::gql::mutations::MutationRoot;

async fn build_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    // get mongodb datasource. It can be added to:
    // 1. As global data for async-graphql.
    // 2. As application scope state of Tide
    // 3. Use lazy-static.rs.
    let mongo_ds = mongo::DataSource::init().await;

    // The root object for the query and Mutatio, and use EmptySubscription.
    // Add global mongodb datasource  in the schema object.
    // let mut schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription)
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(mongo_ds)
        .finish()
}

pub async fn graphql(req: Request<State>) -> tide::Result {
    let schema = self::build_schema().await;

    let token = req
        .header("token")
        .and_then(|values| values.get(0))
        .map(|value| value.as_str().to_string());

    let mut req = async_graphql_tide::receive_request(req).await.unwrap();
    if let Some(token) = token {
        req = req.data(token);
    }

    async_graphql_tide::respond(schema.execute(req).await)
}

// // If you don't want to use crate async-graphql-tide
// pub async fn graphql(req: Request<State>) -> tide::Result {
//     let schema = req.state().0.clone();
//     let gql_resp = schema.execute(receive_json(req).await?).await;

//     let mut resp = Response::new(StatusCode::Ok);
//     resp.set_body(Body::from_json(&gql_resp)?);

//     Ok(resp.into())
// }

pub async fn graphiql(_: Request<State>) -> tide::Result {
    let mut resp = Response::new(StatusCode::Ok);
    resp.set_body(playground_source(GraphQLPlaygroundConfig::new(
        CFG.get("GRAPHQL_VER").unwrap(),
    )));
    resp.set_content_type(mime::HTML);

    Ok(resp.into())
}
