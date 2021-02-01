use async_graphql::Context;
use bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::users::{
    self,
    models::{User, SignInfo},
};
use crate::articles::{self, models::Article};
use crate::categories::{self, models::Category};
use crate::topics::{self, models::Topic};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by id
    async fn user_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_by_id(db, &id).await
    }

    // get user info by email
    async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_by_email(db, &email).await
    }

    // get user info by username
    async fn user_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_by_username(db, &username).await
    }

    async fn user_sign_in(
        &self,
        ctx: &Context<'_>,
        signature: String,
        password: String,
    ) -> GqlResult<SignInfo> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_sign_in(db, &signature, &password).await
    }

    // Get all Users,
    async fn users_list(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::users_list(db, &token).await
    }

    // Get all articles
    async fn articles_list(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::articles_list(db).await
    }

    // Get all articles of one user by user_id
    async fn articles_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::articles_by_user_id(db, &user_id).await
    }

    // Get all articles of one user by username
    async fn articles_by_username(
        &self,
        ctx: &Context<'_>,
        username: String,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::articles_by_username(db, &username).await
    }

    // Get all articles by category_id
    async fn articles_by_category_id(
        &self,
        ctx: &Context<'_>,
        category_id: ObjectId,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::articles_by_category_id(db, &category_id).await
    }

    // Get article by its slug
    async fn article_by_slug(
        &self,
        ctx: &Context<'_>,
        username: String,
        slug: String,
    ) -> GqlResult<Article> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::article_by_slug(db, &username, &slug).await
    }

    // Get all categories
    async fn categories_list(
        &self,
        ctx: &Context<'_>,
    ) -> GqlResult<Vec<Category>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        categories::services::categories_list(db).await
    }

    // Get category by its id
    async fn category_by_id(
        &self,
        ctx: &Context<'_>,
        id: ObjectId,
    ) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        categories::services::category_by_id(db, &id).await
    }

    // Get category by its slug
    async fn category_by_slug(
        &self,
        ctx: &Context<'_>,
        slug: String,
    ) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        categories::services::category_by_slug(db, &slug).await
    }

    // search topics by article_id
    async fn topics_by_article_id(
        &self,
        ctx: &Context<'_>,
        article_id: ObjectId,
    ) -> GqlResult<Vec<Topic>> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        topics::services::topics_by_article_id(db, &article_id).await
    }
}