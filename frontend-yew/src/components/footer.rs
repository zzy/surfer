use yew::prelude::*;
use yew_functional::*;

#[function_component(Copyright)]
pub fn copyright() -> Html {
    html! {
        // Please customize your footer
        <footer class="ps-relative t96 py32 ta-center fc-black-300 bg-black-025">
            <p>
                { "Powered by "}
                <a href="//github.com/zzy/rusthub" target="_blank">{ "zzy/rusthub" }</a>
            </p>
            <p>
                { "~ 版权所有（" }
                <a href="mailto:linshi@budshome.com">{ "linshi@budshome.com" }</a>
                { "）~" }<br />
                { "~" }
                <a href="//rusthub.top" target="_blank">{ "锈毂 - RustHub.top" }</a>
                { " ~"}
            </p>
            <p>
                <a href="#">{ "Back to top" }</a>
            </p>
        </footer>
    }
}

#[function_component(LoadJs)]
pub fn load_js() -> Html {
    html! {
        // load scripts
        <script src="/js/load.js?132689068675031052" defer=true></script>
    }
}
