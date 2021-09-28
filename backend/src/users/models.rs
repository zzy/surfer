use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::util::constant::{GqlResult, DT_F};
use crate::dbs::mongo::DataSource;
use crate::articles::{models::Article, services::articles_by_user_id};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub picture: String,
    pub cred: String,
    pub blog_name: String,
    pub website: String,
    pub introduction: String,
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

    pub async fn nickname(&self) -> &str {
        self.nickname.as_str()
    }

    pub async fn picture(&self) -> &str {
        self.picture.as_str()
    }

    pub async fn blog_name(&self) -> &str {
        self.blog_name.as_str()
    }

    pub async fn website(&self) -> &str {
        self.website.as_str()
    }

    pub async fn introduction(&self) -> &str {
        self.introduction.as_str()
    }

    pub async fn created_at(&self) -> String {
        self.created_at
            .to_chrono()
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(DT_F)
            .to_string()
    }

    pub async fn updated_at(&self) -> String {
        self.updated_at
            .to_chrono()
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(DT_F)
            .to_string()
    }

    pub async fn banned(&self) -> bool {
        self.banned
    }

    pub async fn articles(
        &self,
        ctx: &async_graphql::Context<'_>,
        published: i32,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles_by_user_id(db, self._id, published).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct UserNew {
    pub email: String,
    pub username: String,
    pub nickname: String,
    pub picture: String,
    pub cred: String,
    pub blog_name: String,
    pub website: String,
    pub introduction: String,
    #[graphql(skip)]
    pub banned: bool,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Wish {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    pub published: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::Object]
impl Wish {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn user_id(&self) -> ObjectId {
        self.user_id.clone()
    }

    pub async fn aphorism(&self) -> &str {
        self.aphorism.as_str()
    }

    pub async fn author(&self) -> &str {
        self.author.as_str()
    }

    pub async fn published(&self) -> bool {
        self.published
    }

    pub async fn created_at(&self) -> String {
        self.created_at
            .to_chrono()
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(DT_F)
            .to_string()
    }

    pub async fn updated_at(&self) -> String {
        self.updated_at
            .to_chrono()
            .with_timezone(&FixedOffset::east(8 * 3600))
            .format(DT_F)
            .to_string()
    }

    pub async fn user(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        super::services::user_by_id(db, self.user_id).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct WishNew {
    pub user_id: ObjectId,
    pub aphorism: String,
    pub author: String,
    #[graphql(skip)]
    pub published: bool,
}
