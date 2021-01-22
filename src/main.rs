mod util;
mod dbs;
mod gql;
mod routes;

mod users;
mod articles;

use crate::util::constant::CFG;
use crate::routes::push_routes;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app_state = tide::with_state(State(gql::build_schema().await));
    app_state = push_routes(app_state).await;

    app_state
        .listen(format!(
            "{}:{}",
            CFG.get("ADDRESS").unwrap(),
            CFG.get("PORT").unwrap()
        ))
        .await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State(
    pub  async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
);
