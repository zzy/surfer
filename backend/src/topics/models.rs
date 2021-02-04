use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub _id: ObjectId,
    pub name: String,
    pub quotes: i64,
    pub slug: String,
    pub uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::Object]
impl Topic {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn quotes(&self) -> i64 {
        self.quotes
    }

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub async fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    pub async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicNew {
    pub name: String,
    #[graphql(skip)]
    pub quotes: i64,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TopicArticle {
    pub _id: ObjectId,
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}

#[async_graphql::Object]
impl TopicArticle {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn article_id(&self) -> ObjectId {
        self.article_id.clone()
    }

    pub async fn topic_id(&self) -> ObjectId {
        self.topic_id.clone()
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicArticleNew {
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}
