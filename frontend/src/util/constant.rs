use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "PROTOCOL",
            dotenv::var("PROTOCOL").expect("Expected PROTOCOL to be set in env!"),
        );
        map.insert(
            "ADDRESS",
            dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!"),
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
            "GRAPHQL_URI",
            dotenv::var("GRAPHQL_URI").expect("Expected GRAPHQL_URI to be set in env!"),
        );
        map.insert(
            "GRAPHQL_VER",
            dotenv::var("GRAPHQL_VER").expect("Expected GRAPHQL_VER to be set in env!"),
        );
        map.insert(
            "GRAPHIQL_VER",
            dotenv::var("GRAPHIQL_VER").expect("Expected GRAPHIQL_VER to be set in env!"),
        );

        map.insert(
            "MONGODB_URI",
            dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_BLOG",
            dotenv::var("MONGODB_BLOG").expect("Expected MONGODB_BLOG to be set in env!"),
        );

        map
    };
}
