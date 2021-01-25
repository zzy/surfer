use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

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
            "GQL_PROT",
            dotenv::var("GQL_PROT").expect("Expected GQL_PROT to be set in env!"),
        );
        map.insert(
            "GQL_ADDR",
            dotenv::var("GQL_ADDR").expect("Expected GQL_ADDR to be set in env!"),
        );
        map.insert(
            "GQL_PORT",
            dotenv::var("GQL_PORT").expect("Expected GQL_PORT to be set in env!"),
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

        map
    };
}
