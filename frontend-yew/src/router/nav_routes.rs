use yew::prelude::*;
use yew_router::prelude::*;

use crate::show::{
    home::Home, articles::Articles, categories::Categories, topics::Topics,
    explore::Explore,
};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum NavRoutes {
    #[at("/")]
    Home,
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
