use async_graphql::Context;
use bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::users::{
    self,
    models::{User, SignInfo},
};
use crate::articles::{self, models::Article};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by id
    async fn user_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_by_id(db, &id).await
    }

    // get user info by email
    async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_by_email(db, &email).await
    }

    // get user info by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_by_username(db, &username).await
    }

    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        autograph: String,
        password: String,
    ) -> GqlResult<SignInfo> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_sign_in(db, &autograph, &password).await
    }

    // Get all Users,
    async fn users_list(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::users_list(db, &token).await
    }

    // Get all articles
    async fn articles_list(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::articles_list(db).await
    }

    // Get all articles of one username
    async fn articles_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::articles_by_username(db, &username).await
    }

    // Get all articles of one article's slug
    async fn article_by_slug(
        &self,
        ctx: &Context<'_>,
        username: String,
        slug: String,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::article_by_slug(db, &username, &slug).await
    }
}
