use tide::{self, Server, Request};
use serde_json::json;

pub mod users;
pub mod articles;

use crate::State;
use crate::util::common::Tpl;

use crate::routes::{
    users::{users_list, user_register},
    articles::{articles_list, article_new},
};

pub async fn push_res(mut app: Server<State>) -> Server<State> {
    app.at("/static").serve_dir("static").unwrap();

    app.at("/").get(index);

    let mut users = app.at("users");
    users.at("list").get(users_list);
    users.at("register").get(user_register);

    let mut articles = app.at("articles");
    articles.at("list").get(articles_list);
    articles.at("new").get(article_new);

    app
}

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "blog-rs", "author": "zzy"});

    index.render(&data).await
}
