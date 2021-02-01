use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

use crate::util::constant::GqlResult;
use crate::dbs::mongo::DataSource;
use crate::articles::{models::Article, services::articles_by_category_id};

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub uri: String,
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

    pub async fn slug(&self) -> &str {
        self.slug.as_str()
    }

    pub async fn uri(&self) -> &str {
        self.uri.as_str()
    }

    pub async fn articles(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles_by_category_id(db, &self._id).await
    }
}

#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct CategoryNew {
    pub name: String,
    pub description: String,
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
