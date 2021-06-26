use yew::prelude::*;

pub struct Home;

impl Component for Home {
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
        let home_cls = "home";

        html! {
            <div class=classes!(home_cls)>
               <h1>{ "Rust + WebAssembly" }</h1>
               <h4>{ "使用 yew 构建 WebAssembly 标准的 web 前端" }</h4>
            </div>
        }
    }
}
