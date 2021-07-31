#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod util;
mod components;
mod router;
mod show;
mod manage;

use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::nav_routes::{NavRoutes, switch};
use crate::components::{header::*, footer::*};

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
        html! {
            <>
                <Header />

                <main class="ps-relative t64">
                    <Router<NavRoutes> render={ Router::render(switch) } />
                </main>

                <Copyright />

                <LoadJs />
            </>
        }
    }
}

fn main() {
    set_panic_hook();

    yew::start_app::<App>();
}
