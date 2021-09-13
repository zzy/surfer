use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

// async-graphql result type
pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

// datetime format
pub const DT_F: &str = "%Y-%m-%d %H:%M:%S%Z";

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDR",
            dotenv::var("ADDR").expect("Expected ADDR to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );

        map.insert(
            "SITE_KEY",
            dotenv::var("SITE_KEY").expect("Expected SITE_KEY to be set in env!"),
        );
        map.insert(
            "CLAIM_EXP",
            dotenv::var("CLAIM_EXP").expect("Expected CLAIM_EXP to be set in env!"),
        );

        map.insert(
            "GQL_URI",
            dotenv::var("GQL_URI").expect("Expected GQL_URI to be set in env!"),
        );
        map.insert(
            "GQL_VER",
            dotenv::var("GQL_VER").expect("Expected GQL_VER to be set in env!"),
        );
        map.insert(
            "GIQL_VER",
            dotenv::var("GIQL_VER").expect("Expected GIQL_VER to be set in env!"),
        );

        map.insert(
            "MONGODB_URI",
            dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_NAME",
            dotenv::var("MONGODB_NAME").expect("Expected MONGODB_NAME to be set in env!"),
        );

        map
    };
}
