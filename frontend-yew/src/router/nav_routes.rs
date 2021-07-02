use yew::prelude::*;
use yew_router::prelude::*;

use crate::show::{
    home::Home, sign_in::SignIn, register::Register, articles::Articles,
    categories::Categories, topics::Topics, explore::Explore,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum NavRoutes {
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
}

pub fn switch(goal: &NavRoutes) -> Html {
    match goal {
        NavRoutes::Home => {
            html! { <Home /> }
        }
        NavRoutes::SignIn => {
            html! { <SignIn /> }
        }
        NavRoutes::Register => {
            html! { <Register /> }
        }
        NavRoutes::Articles => {
            html! { <Articles /> }
        }
        NavRoutes::Categories => {
            html! { <Categories /> }
        }
        NavRoutes::Topics => {
            html! { <Topics /> }
        }
        NavRoutes::Explore => {
            html! { <Explore /> }
        }
    }
}
