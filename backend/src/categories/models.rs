use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::util::constant::{GqlResult, DT_F};
use crate::dbs::mongo::DataSource;
use crate::{
    articles::{models::Article, services::articles_by_category_id},
    topics::{models::Topic, services::topics_by_category_id},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub quotes: i64,
    pub slug: String,
    pub uri: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[async_graphql::Object]
impl Category {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn description(&self) -> &str {
        self.description.as_str()
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

    pub async fn articles(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        articles_by_category_id(db, self._id, 1).await
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics_by_category_id(db, self._id, 1).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct CategoryNew {
    pub name: String,
    pub description: String,
    #[graphql(skip)]
    pub quotes: i64,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CategoryUser {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}

#[async_graphql::Object]
impl CategoryUser {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn user_id(&self) -> ObjectId {
        self.user_id.clone()
    }

    pub async fn category_id(&self) -> ObjectId {
        self.category_id.clone()
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct CategoryUserNew {
    pub user_id: ObjectId,
    pub category_id: ObjectId,
}
