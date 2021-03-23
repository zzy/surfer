use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type GqlResult<T> = std::result::Result<T, async_graphql::Error>;

lazy_static! {
    // datetime format
    pub static ref DT_F: String = String::from("%Y-%m-%d %H:%M:%S%Z");

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

        // map.insert(
        //     "WEB_PROT",
        //     dotenv::var("WEB_PROT").expect("Expected WEB_PROT to be set in env!"),
        // );
        // map.insert(
        //     "WEB_ADDR",
        //     dotenv::var("WEB_ADDR").expect("Expected WEB_ADDR to be set in env!"),
        // );
        // map.insert(
        //     "WEB_PORT",
        //     dotenv::var("WEB_PORT").expect("Expected WEB_PORT to be set in env!"),
        // );

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
            "MONGODB_BLOG",
            dotenv::var("MONGODB_BLOG").expect("Expected MONGODB_BLOG to be set in env!"),
        );

        map
    };
}
