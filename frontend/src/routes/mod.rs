use tide::{self, Server};

pub mod home;
pub mod users;
pub mod articles;

use crate::State;

use crate::routes::home::{index, register, sign_in, sign_out};
use crate::routes::users::{user_index, user_dashboard, users_list, user_register};
use crate::routes::articles::{article_index, articles_list, article_new};

// pub async fn push_res(mut app: Server<State>) -> Server<State> {
pub async fn push_res(app: &mut Server<State>) {
    app.at("/static").serve_dir("./static").unwrap();

    let mut home = app.at("");
    home.at("/").get(index);

    home.at("/register").get(register).post(register);
    home.at("/sign-in").get(sign_in).post(sign_in);
    home.at("/sign-out").get(sign_out);

    home.at("/:username").get(user_index);
    home.at("/:username/dashboard").get(user_dashboard);
    home.at("/:username/:slug").get(article_index);

    let mut users = app.at("users");
    users.at("list").get(users_list);
    users.at("register").get(user_register);

    let mut articles = app.at("articles");
    articles.at("list").get(articles_list);
    articles.at("new").get(article_new).post(article_new);

    // app
}
