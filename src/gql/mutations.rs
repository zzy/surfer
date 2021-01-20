use async_graphql::Context;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::users::{
    self,
    models::{User, NewUser},
};
use crate::articles::{
    self,
    models::{Article, NewArticle},
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(&self, ctx: &Context<'_>, new_user: NewUser) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_register(db, new_user).await
    }

    // Change user password
    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        cur_password: String,
        new_password: String,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_change_password(db, &cur_password, &new_password, &token).await
    }

    // update user profile
    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        new_user: NewUser,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_update_profile(db, new_user, &token).await
    }

    // Add new article
    async fn add_article(&self, ctx: &Context<'_>, new_article: NewArticle) -> Article {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        articles::services::add_article(db, new_article).await
    }
}
