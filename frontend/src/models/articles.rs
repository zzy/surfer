use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleInfo {
    pub content: String,
}
