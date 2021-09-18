use yew::prelude::*;
use yew_router::prelude::*;

use crate::show::{
    home::Home, sign_in::SignIn, register::Register, author::Author,
    articles::Articles, categories::Categories, topics::Topics,
    article::Article, category::Category, topic::Topic, explore::Explore,
};
use crate::manage::manage_index::ManageIndex;
use crate::components::nodes::page_not_found;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Routes {
    /////////
    // nav //
    /////////
    #[at("/")]
    Home,
    #[at("/sign-in")]
    SignIn,
    #[at("/register")]
    Register,
    #[at("/articles")]
    Articles,
    #[at("/categories")]
    Categories,
    #[at("/topics")]
    Topics,
    #[at("/explore")]
    Explore,
    ///////////
    // items //
    ///////////
    #[at("/:username")]
    Author { username: String },
    #[at("/:username/:article_slug")]
    Article { username: String, article_slug: String },
    #[at("/category/:category_slug")]
    Category { category_slug: String },
    #[at("/topic/:topic_slug")]
    Topic { topic_slug: String },
    ////////////
    // manage //
    ////////////
    #[at("/manage")]
    ManageIndex,
    ////////////
    // erros //
    ////////////
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(goal: &Routes) -> Html {
    match goal {
        /////////
        // nav //
        /////////
        Routes::Home => {
            html! { <Home /> }
        }
        Routes::SignIn => {
            html! { <SignIn /> }
        }
        Routes::Register => {
            html! { <Register /> }
        }
        Routes::Articles => {
            html! { <Articles /> }
        }
        Routes::Categories => {
            html! { <Categories /> }
        }
        Routes::Topics => {
            html! { <Topics /> }
        }
        Routes::Explore => {
            html! { <Explore /> }
        }
        ///////////
        // items //
        ///////////
        Routes::Author { username } => {
            html! { <Author username={username.clone()} /> }
        }
        Routes::Article { username, article_slug } => {
            html! { <Article username={username.clone()} article_slug={article_slug.clone()} /> }
        }
        Routes::Category { category_slug } => {
            html! { <Category category_slug={category_slug.clone()} /> }
        }
        Routes::Topic { topic_slug } => {
            html! { <Topic topic_slug={topic_slug.clone()} /> }
        }
        ////////////
        // manage //
        ////////////
        Routes::ManageIndex => {
            html! { <ManageIndex /> }
        }
        ////////////
        // erros //
        ////////////
        Routes::NotFound => page_not_found(),
    }
}
