use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub subject: String,
    pub slug: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub published: bool,
}

#[async_graphql::Object]
impl Article {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn user_id(&self) -> ObjectId {
        self.user_id.clone()
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
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct ArticleNew {
    pub user_id: ObjectId,
    pub subject: String,
    #[graphql(skip)]
    pub slug: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[graphql(skip)]
    pub published: bool,
}
