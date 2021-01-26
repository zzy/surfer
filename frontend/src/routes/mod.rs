use tide::{self, Server, Request};

pub mod users;
pub mod articles;

use crate::State;
use crate::util::common::{rhai_dir, Tpl};

use crate::routes::{
    users::{users_list, user_register, user_index},
    articles::{articles_list, article_new},
};

async fn index(_req: Request<State>) -> tide::Result {
    let mut index: Tpl = Tpl::new("index").await;
    index.reg.register_script_helper_file(
        "blog-name",
        format!("{}{}", rhai_dir().await, "blog-name.rhai"),
    )?;

    let data = ();

    index.render(&data).await
}

pub async fn push_res(mut app: Server<State>) -> Server<State> {
    app.at("/").serve_dir("static").unwrap();

    app.at("/").get(index);
    app.at("/:username").get(user_index);

    let mut users = app.at("users");
    users.at("list").get(users_list);
    users.at("register").get(user_register);

    let mut articles = app.at("articles");
    articles.at("list").get(articles_list);
    articles.at("new").get(article_new);

    app
}
