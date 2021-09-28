use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::util::constant::{GqlResult, DT_F};
use crate::dbs::mongo::DataSource;
use crate::articles::{models::Article, services::articles_by_topic_id};

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
        articles_by_topic_id(db, self._id, 1).await
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
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TopicArticle {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}

#[async_graphql::Object]
impl TopicArticle {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn user_id(&self) -> ObjectId {
        self.user_id.clone()
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
    pub user_id: ObjectId,
    pub article_id: ObjectId,
    pub topic_id: ObjectId,
}
