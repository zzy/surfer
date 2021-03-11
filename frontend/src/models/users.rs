use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInInfo {
    pub signature: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub blog_name: String,
    pub website: String,
    pub introduction: String,
}

#[derive(Deserialize)]
pub struct ArticleInfo {
    pub content: String,
}
