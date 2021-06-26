use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub gql: Gql,
    pub theme_mode: ThemeMode,
    pub i18n: I18n,
    pub github: Github,
}

#[derive(Deserialize)]
pub struct Gql {
    pub addr: String,
    pub port: u16,
    pub path: String,
}

#[derive(Deserialize)]
pub struct ThemeMode {
    pub title: String,
    pub svg: String,
}

#[derive(Deserialize)]
pub struct I18n {
    pub title: String,
    pub href: String,
    pub svg: String,
}

#[derive(Deserialize)]
pub struct Github {
    pub title: String,
    pub href: String,
    pub svg: String,
}
