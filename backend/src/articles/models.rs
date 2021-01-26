use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub _id: ObjectId,
    pub username: String,
    pub subject: String,
    pub slug: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub published: bool,
    pub top: bool,
    pub recommended: bool,
}

#[async_graphql::Object]
impl Article {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }

    pub async fn subject(&self) -> &str {
        self.subject.as_str()
    }

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn content(&self) -> &str {
        self.content.as_str()
    }

    pub async fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime {
        self.updated_at
    }

    pub async fn published(&self) -> bool {
        self.published
    }

    pub async fn top(&self) -> bool {
        self.top
    }

    pub async fn recommended(&self) -> bool {
        self.recommended
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct ArticleNew {
    pub username: String,
    pub subject: String,
    #[graphql(skip)]
    pub slug: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub published: bool,
    #[graphql(skip)]
    pub top: bool,
    #[graphql(skip)]
    pub recommended: bool,
}
