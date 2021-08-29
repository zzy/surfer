use yew::prelude::*;

pub struct SignIn;

impl Component for SignIn {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="m24 p8 fs-subheading bg-blue-100">
                { "--- Sign in, Work In Progress ---" }
            </div>
        }
    }
}
