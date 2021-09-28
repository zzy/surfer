use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, DateTime};
use chrono::FixedOffset;

use crate::util::constant::{GqlResult, DT_F};
use crate::dbs::mongo::DataSource;
use crate::categories::{self, models::Category};
use crate::topics::{self, models::Topic};
use crate::users::{self, models::User};

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

    pub async fn content_html(&self) -> String {
        use pulldown_cmark::{Parser, Options, html};

        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(&self.content, options);

        let mut content_html = String::new();
        html::push_html(&mut content_html, parser);

        content_html
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
        users::services::user_by_id(db, self.user_id).await
    }

    pub async fn category(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        categories::services::category_by_id(db, self.category_id).await
    }

    pub async fn topics(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db.clone();
        topics::services::topics_by_article_id(db, self._id).await
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
}
