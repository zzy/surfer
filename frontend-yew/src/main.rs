#![recursion_limit = "1024"]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod util;
mod components;
mod show;
mod manage;

use console_error_panic_hook::set_once as set_panic_hook;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::routes::{Routes, switch};
use crate::components::{header::*, footer::*};

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />

                <main class="ps-relative t64">
                    <Switch<Routes> render={ Switch::render(switch) } />
                </main>

                <Copyright />

                <LoadJs />
            </BrowserRouter>
        }
    }
}

fn main() {
    set_panic_hook();

    yew::start_app::<App>();
}
