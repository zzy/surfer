use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Topic {
    pub _id: ObjectId,
    pub name: String,
    pub article_id: ObjectId,
    pub slug: String,
    pub uri: String,
    // pub created_at: DateTime,
    // pub updated_at: DateTime,
}

#[async_graphql::Object]
impl Topic {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn article_id(&self) -> ObjectId {
        self.article_id.clone()
    }

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn uri(&self) -> &str {
        self.uri.as_str()
    }

    // pub async fn created_at(&self) -> DateTime {
    //     self.created_at
    // }

    // pub async fn updated_at(&self) -> DateTime {
    //     self.updated_at
    // }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct TopicNew {
    pub name: String,
    pub article_id: ObjectId,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
    // pub created_at: DateTime,
    // pub updated_at: DateTime,
}
