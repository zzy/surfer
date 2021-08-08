use yew::prelude::*;

pub struct Explore;

impl Component for Explore {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="m24 p8 fs-subheading bg-blue-100">
                { "--- Explore, Work In Progress ---" }
                <div class="mt12 fw-bold">
                    { "See also: " }
                    <a href="https://blog.budshome.com" target="_blank">
                        { "Rust 技术博客（graphql + handlebars）" }
                    </a>
                </div>
            </div>
        }
    }
}
