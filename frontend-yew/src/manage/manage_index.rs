use yew::prelude::*;

pub struct ManageIndex;

impl Component for ManageIndex {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "--- ManageIndex ---" }</h1>
            </>
        }
    }
}
