mod util;
mod dbs;
mod gql;

mod users;
mod articles;

use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::with_state(State {});

    //environment variables defined in .env file
    let mut gql = app.at(CFG.get("GRAPHQL_URI").unwrap());
    gql.at(CFG.get("GRAPHQL_VER").unwrap()).post(gql::graphql);
    gql.at(CFG.get("GRAPHIQL_VER").unwrap()).get(gql::graphiql);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State {}
