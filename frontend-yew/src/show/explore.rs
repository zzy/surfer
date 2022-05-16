use yew::prelude::*;

pub struct Explore;

impl Component for Explore {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="m24 p8 fs-subheading bg-blue-100">
                { "--- Explore, Work In Progress ---" }
                <div class="mt12 fw-bold">
                    { "See also: " }
                    <a href="https://blog.ruonou.com" target="_blank">
                        { "Rust 技术博客（graphql + handlebars）" }
                    </a>
                </div>
            </div>
        }
    }
}
