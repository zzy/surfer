use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub _id: ObjectId,
    pub name: String,
    pub slug: String,
    pub uri: String,
}

#[async_graphql::Object]
impl Topic {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn uri(&self) -> &str {
        self.uri.as_str()
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicNew {
    pub name: String,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
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
