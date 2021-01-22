use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime};

use crate::util::constant::GqlResult;
use crate::dbs::mongo::DataSource;
use crate::articles::{models::Article, services::articles_by_user};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub username: String,
    pub cred: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub banned: bool,
}

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }

    pub async fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime {
        self.updated_at
    }

    pub async fn banned(&self) -> bool {
        self.banned
    }

    pub async fn articles(&self, ctx: &async_graphql::Context<'_>) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles_by_user(db, self._id.clone()).await
    }
}
#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct UserNew {
    pub email: String,
    pub username: String,
    pub cred: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub banned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInfo {
    pub email: String,
    pub username: String,
    pub token: String,
}

#[async_graphql::Object]
impl SignInfo {
    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }

    pub async fn token(&self) -> &str {
        self.token.as_str()
    }
}
