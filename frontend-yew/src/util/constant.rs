use std::collections::HashMap;
use toml::from_str;
use serde::Deserialize;
use lazy_static::lazy_static;

lazy_static! {
    // CFG variables defined in cfg.toml file
    pub static ref CFG: HashMap<&'static str, String> = {
        let cfg_str = include_str!("../../cfg.toml");
        let config: Config = from_str(cfg_str).unwrap();

        let mut map = HashMap::new();

        map.insert("addr", config.gql.addr);
        map.insert("port", config.gql.port.to_string());
        map.insert("path",config.gql.path);

        map
    };
}

#[derive(Deserialize)]
struct Config {
    gql: Gql,
}

#[derive(Deserialize)]
struct Gql {
    addr: String,
    port: u16,
    path: String,
}
