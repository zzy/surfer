use yew::prelude::*;

pub struct Register;

impl Component for Register {
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
                { "--- Register, Work In Progress ---" }
            </div>
        }
    }
}
