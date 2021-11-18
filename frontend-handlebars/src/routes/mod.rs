use tide::{self, Server};

pub mod home;
pub mod users;
pub mod articles;

use crate::State;
use crate::util::common::tpls_dir;

use crate::routes::home::{index, register, sign_in, sign_out};
use crate::routes::users::{user_index, user_dashboard, users_list};
use crate::routes::articles::{article_index, articles_list, article_new};

pub async fn push_res(app: &mut Server<State>) {
    app.at("/").get(index);
    app.at("/static/*").serve_dir("./static/").unwrap();
    app.at("/ads.txt")
        .serve_file(format!("{}{}", tpls_dir().await, "ads.txt"))
        .unwrap();

    let mut home = app.at("/");
    home.at("/register").get(register).post(register);
    home.at("/sign-in").get(sign_in).post(sign_in);
    home.at("/sign-out").get(sign_out);

    let mut user = app.at("/:username");
    user.at("/").get(user_index);
    user.at("/dashboard").get(user_dashboard);
    user.at("/:slug").get(article_index);

    let mut users = app.at("/users");
    users.at("/").get(users_list);

    let mut articles = app.at("/articles");
    articles.at("/").get(articles_list);
    articles.at("/new").get(article_new).post(article_new);
}
