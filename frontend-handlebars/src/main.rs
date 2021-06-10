mod util;
mod routes;
mod models;

use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    // Something in Tide State
    let app_state = State {};
    let mut app = tide::with_state(app_state);
    // app = push_res(app).await;
    routes::push_res(&mut app).await;

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDR").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State {}
