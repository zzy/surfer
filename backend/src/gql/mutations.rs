use async_graphql::Context;

use crate::dbs::mongo::DataSource;
use crate::util::constant::GqlResult;
use crate::users::{
    self,
    models::{User, UserNew},
};
use crate::articles::{
    self,
    models::{Article, ArticleNew},
};
use crate::categories::{
    self,
    models::{Category, CategoryNew, CategoryUser, CategoryUserNew},
};
use crate::topics::{
    self,
    models::{Topic, TopicNew},
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_register(db, user_new).await
    }

    // Change user password
    async fn user_change_password(
        &self,
        ctx: &Context<'_>,
        pwd_cur: String,
        pwd_new: String,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_change_password(db, &pwd_cur, &pwd_new, &token)
            .await
    }

    // update user profile
    async fn user_update_profile(
        &self,
        ctx: &Context<'_>,
        user_new: UserNew,
        token: String,
    ) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        users::services::user_update_profile(db, user_new, &token).await
    }

    // Add new article
    async fn article_new(
        &self,
        ctx: &Context<'_>,
        article_new: ArticleNew,
    ) -> GqlResult<Article> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        articles::services::article_new(db, article_new).await
    }

    // Add new category
    async fn category_new(
        &self,
        ctx: &Context<'_>,
        category_new: CategoryNew,
    ) -> GqlResult<Category> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        categories::services::category_new(db, category_new).await
    }

    // Add new category
    async fn category_user_new(
        &self,
        ctx: &Context<'_>,
        category_user_new: CategoryUserNew,
    ) -> GqlResult<CategoryUser> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        categories::services::category_user_new(db, category_user_new).await
    }

    // Add new topic
    async fn topic_new(
        &self,
        ctx: &Context<'_>,
        topic_new: TopicNew,
    ) -> GqlResult<Topic> {
        let db = ctx.data_unchecked::<DataSource>().db_blog.clone();
        topics::services::topic_new(db, topic_new).await
    }
}
