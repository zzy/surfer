use std::collections::HashMap;
use toml::from_str;
use lazy_static::lazy_static;

use super::models::*;

pub type ObjectId = String;

lazy_static! {
    // CFG variables defined in cfg.toml file
    pub static ref CFG: HashMap<&'static str, String> = {
        let cfg_str = include_str!("../../.env.toml");
        let config: Config = from_str(cfg_str).unwrap();

        let mut map = HashMap::new();

        map.insert("site.title", config.site.title);

        map.insert("gql.addr", config.gql.addr);
        map.insert("gql.path",config.gql.path);

        map.insert("theme_mode.title", config.theme_mode.title);
        map.insert("theme_mode.svg", config.theme_mode.svg);

        map.insert("i18n.title", config.i18n.title);
        map.insert("i18n.href", config.i18n.href);
        map.insert("i18n.svg",config.i18n.svg);

        map.insert("github.title", config.github.title);
        map.insert("github.href", config.github.href);
        map.insert("github.svg",config.github.svg);

        map
    };
}
