mod util;
mod routes;

use crate::util::constant::CFG;
use crate::routes::push_res;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let app_state = State {};
    let mut app = tide::with_state(app_state);
    app = push_res(app).await;

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
