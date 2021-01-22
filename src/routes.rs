use tide::{self, Server, Request};
use serde_json::json;

use crate::State;
use crate::util::{constant::CFG, common::Tpl};

use crate::gql::{graphiql, graphql};
use crate::articles::routes::{articles_list, article_register};
use crate::users::routes::{users_list, user_register};

pub async fn push_routes(mut app_state: Server<State>) -> Server<State> {
    // let mut app_routes = app_state;
    app_state.at("/").get(index);

    let mut gql = app_state.at(CFG.get("GRAPHQL_URI").unwrap());
    // app.at(ENV.get("GRAPHQL_VER").unwrap()).post(async_graphql_tide::endpoint(schema));
    gql.at(CFG.get("GRAPHQL_VER").unwrap()).post(graphql);
    gql.at(CFG.get("GRAPHIQL_VER").unwrap()).get(graphiql);

    let mut users = app_state.at("users");
    users.at("list").get(users_list);
    users.at("register").get(user_register);

    let mut articles = app_state.at("articles");
    articles.at("list").get(articles_list);
    articles.at("new").get(article_register);

    app_state
}

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "blog-rs", "author": "zzy"});

    index.render(&data).await
}
