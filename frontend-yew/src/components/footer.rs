use yew::prelude::*;

#[function_component(Copyright)]
pub fn copyright() -> Html {
    html! {
        // Please customize your footer
        <footer class="ps-relative t96 py32 ta-center fc-black-300 bg-black-025">
            <p>
                { "Powered by "}
                <a href="//github.com/zzy/surfer" target="_blank">{ "zzy/surfer" }</a>
            </p>
            <p>
                { "~ 版权所有（" }
                <a href="mailto:ask@ruonou.com">{ "ask@ruonou.com" }</a>
                { "）~" }<br />
                { "~ " }
                <a href="//ruonou.com" target="_blank">{ "若耨 - RuoNou.top" }</a>
                { " ~"}
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
