use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArticleInfo {
    pub user_id: String,
    pub subject: String,
    pub category_id: String,
    pub summary: String,
    pub topic_names: String,
    pub content: String,
}
