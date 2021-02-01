use serde::{Serialize, Deserialize};
use bson::{oid::ObjectId, DateTime};

use crate::util::constant::GqlResult;
use crate::dbs::mongo::DataSource;
use crate::categories::services::category_by_id;
use crate::topics::services::topics_by_article_id;

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub subject: String,
    pub category_id: ObjectId,
    pub summary: String,
    pub slug: String,
    pub uri: String,
    pub content: String,
    pub published: bool,
    pub top: bool,
    pub recommended: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
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

    pub async fn category_id(&self) -> ObjectId {
        self.category_id.clone()
    }

    pub async fn summary(&self) -> &str {
        self.summary.as_str()
    }

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub async fn content(&self) -> &str {
        self.content.as_str()
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

    pub async fn created_at(&self) -> DateTime {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime {
        self.updated_at
    }

    pub async fn category_name(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<String> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        let category = category_by_id(db.clone(), &self.category_id).await?;

        Ok(category.name)
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<String>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        let topics_list = topics_by_article_id(db, &self._id).await?;

        let mut topics = Vec::new();
        for topic in topics_list {
            topics.push(topic.name);
        }

        Ok(topics)
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct ArticleNew {
    pub user_id: ObjectId,
    pub subject: String,
    pub category_id: ObjectId,
    pub summary: String,
    #[graphql(skip)]
    pub slug: String,
    #[graphql(skip)]
    pub uri: String,
    pub content: String,
    #[graphql(skip)]
    pub published: bool,
    #[graphql(skip)]
    pub top: bool,
    #[graphql(skip)]
    pub recommended: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
