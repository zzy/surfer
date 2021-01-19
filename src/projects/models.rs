use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub subject: String,
    pub website: String,
}

#[async_graphql::Object]
impl Project {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn user_id(&self) -> ObjectId {
        self.user_id.clone()
    }

    pub async fn subject(&self) -> &str {
        self.subject.as_str()
    }

    pub async fn website(&self) -> &str {
        self.website.as_str()
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct NewProject {
    pub user_id: ObjectId,
    pub subject: String,
    pub website: String,
}
