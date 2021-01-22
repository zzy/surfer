use async_graphql::Context;
use bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::users::{
    self,
    models::{User, UserNew, SignInfo},
};
use crate::articles::{self, models::Article};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by email
    async fn get_user_by_email(&self, ctx: &Context<'_>, email: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::get_user_by_email(db, &email).await
    }

    // get user info by username
    async fn get_user_by_username(&self, ctx: &Context<'_>, username: String) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::get_user_by_username(db, &username).await
    }

    async fn user_sign_in(&self, ctx: &Context<'_>, unknown_user: UserNew) -> GqlResult<SignInfo> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_sign_in(db, unknown_user).await
    }

    // Get all Users,
    async fn users_list(&self, ctx: &Context<'_>, token: String) -> GqlResult<Vec<User>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::users_list(db, &token).await
    }

    // Get all articles
    async fn articles_list(&self, ctx: &Context<'_>) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::articles_list(db).await
    }

    // Get all articles of one User
    async fn articles_by_user(
        &self,
        ctx: &Context<'_>,
        user_id: ObjectId,
    ) -> GqlResult<Vec<Article>> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::articles_by_user(db, user_id).await
    }
}
