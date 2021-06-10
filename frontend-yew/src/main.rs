mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;

use pages::{home::Home, users::Users, projects::Projects};

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum Route {
    #[to = "/users"]
    Users,
    #[to = "/projects"]
    Projects,
    #[to = "/"]
    Home,
}

fn switch(switch: Route) -> Html {
    match switch {
        Route::Users => {
            html! { <Users/> }
        }
        Route::Projects => {
            html! { <Projects/> }
        }
        Route::Home => {
            html! { <Home /> }
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        let home_cls = "nav";

        html! {
            <>
            <div class="logo-title">
                <img src="imgs/budshome.png" />
                { "tide-async-graphql-mongodb / frontend-yew" }
            </div>
            <div class=home_cls>
                <Anchor route=Route::Users>
                    { "用户列表" }
                </Anchor>
                <span class="placeholder">{ " - " }</span>
                <Anchor route=Route::Projects>
                    { "项目列表" }
                </Anchor>
                <span class="placeholder">{ " - " }</span>
                <Anchor route=Route::Home>
                    { "主页" }
                </Anchor>
            </div>
            <main>
                <Router<Route> render=Router::render(switch) />
            </main>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
