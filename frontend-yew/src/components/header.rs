use yew::prelude::*;
use yew_router::prelude::*;

use crate::util::{constant::CFG, routes::Routes};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-black-050 ps-fixed h64 t0 l0 r0 z-nav-fixed
            bb bc-black-3 js-stacks-topbar print:d-none">
            <Nav />
            <NavOver />
        </header>
    }
}

#[function_component(NavGlobal)]
fn nav_global() -> Html {
    html! {
        <>
            <Link<Routes> classes={ classes!("nav-global-articles", "s-navigation--item") }
                to={ Routes::Articles }>
                { "Articles" }
            </Link<Routes>>
            <Link<Routes> classes={ classes!("nav-global-categories", "s-navigation--item") }
            to={ Routes::Categories }>
                { "Categories" }
            </Link<Routes>>
            <Link<Routes> classes={ classes!("nav-global-topics", "s-navigation--item") }
            to={ Routes::Topics }>
                { "Topics" }
            </Link<Routes>>
            <Link<Routes> classes={ classes!("nav-global-explore", "s-navigation--item") }
            to={ Routes::Explore }>
                { "Explore" }
            </Link<Routes>>
            <a class="s-navigation--item" href="//ruonou.com/books.html"
                target="_blank">
                { "Learning" }
            </a>
        </>
    }
}

#[function_component(Sign)]
fn sign() -> Html {
    html! {
        <>
            <Link<Routes> classes={ classes!("nav-sign-signin", "s-btn", "p6") }
                to={ Routes::SignIn }>
                { "Sign in" }
            </Link<Routes>>
            <Link<Routes> classes={ classes!("nav-sign-register", "s-btn", "p6") }
                to={ Routes::Register }>
                { "Register" }
            </Link<Routes>>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <div class="d-flex ai-center px16 h100 mx-auto wmx12 sm:jc-space-between">

            <a class="d-flex flex__center p8 mr8 s-link s-link__muted d-none md:d-block js-hamburger-btn"
                href="#">
                <svg aria-hidden="true" class="svg-icon iconHamburger js-hamburger-icon"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path d="M2 3h14v2H2V3zm0 5h14v2H2V8zm14 5H2v2h14v-2z" />
                </svg>
                <svg aria-hidden="true" class="svg-icon iconClear d-none js-hamburger-close-icon"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path
                        d="M15 4.41 13.59 3 9 7.59 4.41 3 3 4.41 7.59 9 3 13.59 4.41 15 9
                            10.41 13.59 15 15 13.59 10.41 9 15 4.41z" />
                </svg>
            </a>

            <Link<Routes> classes={ classes!("flex--item", "js-logo") } to={ Routes::Home }>
                <span class="fs-title fw-bold">
                    <span class="ff-sans">{ "若耨" }</span>
                    <img class="va-sub" src="/imgs/logos/logo.png"
                        alt={ "RuoNou -- 若耨" } title={ "RuoNou -- 若耨" }
                        aria-label={ "RuoNou -- 若耨" } />
                    <span class="ff-serif">{ "RuoNou" }</span>
                </span>
            </Link<Routes>>

            <nav class="s-navigation mx16 fw-nowrap sm:d-none" aria-label="Global navigation">
                <NavGlobal />
            </nav>

            <button class="s-btn__unset c-pointer flex--item fc-black-300 ml12 sm:d-none
                js-darkmode-btn ml-auto"
                title={ CFG.get("theme_mode.title").unwrap().clone() }>
                <svg aria-hidden="true" class="svg-icon iconTheme"
                    width="24" height="24" viewBox="0 0 18 18">
                    <path d={ CFG.get("theme_mode.svg").unwrap().clone() } />
                </svg>
            </button>

            <a class="flex--item fc-black-300 ml12 sm:d-none"
                title={ CFG.get("i18n.title").unwrap().clone() }
                href={ CFG.get("i18n.href").unwrap().clone() }>
                <svg aria-hidden="true" class="svg-icon iconInternational"
                    width="24" height="24" viewBox="0 0 24 24">
                    <path d={ CFG.get("i18n.svg").unwrap().clone() } />
                </svg>
            </a>

            <a class="flex--item fc-black-300 ml12 sm:d-none"
                title={ CFG.get("github.title").unwrap().clone() }
                href={ CFG.get("github.href").unwrap().clone() } target="_blank">
                <svg aria-hidden="true" class="svg-icon iconGitHub"
                    width="24" height="24" viewBox="0 0 18 18">
                    <path d={ CFG.get("github.svg").unwrap().clone() } />
                </svg>
            </a>

            <div class="flex--item ps-relative ml16 w100 wmx3 sm:wmx-initial sm:ml0 sm:d-none js-search">
                <input id="searchbox" class="s-input s-input__search bar-md js-stacks-search-bar"
                    type="text" placeholder={ "Search RuoNou …" } />
                <svg aria-hidden="true" class="svg-icon iconSearch s-input-icon s-input-icon__search"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path
                        d="m18 16.5-5.14-5.18h-.35a7 7 0 10-1.19 1.19v.35L16.5 18l1.5-1.5zM12
                            7A5 5 0 112 7a5 5 0 0110 0z" />
                </svg>
            </div>

            <a class="d-flex flex__center p8 ml8 s-link s-link__muted d-none sm:d-block js-search-btn"
                href="#">
                <svg aria-hidden="true" class="svg-icon iconSearch js-search-icon"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path
                        d="m18 16.5-5.14-5.18h-.35a7 7 0 10-1.19 1.19v.35L16.5 18l1.5-1.5zM12
                            7A5 5 0 112 7a5 5 0 0110 0z" />
                </svg>
                <svg aria-hidden="true" class="svg-icon iconClear d-none js-search-close-icon"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path
                        d="M15 4.41 13.59 3 9 7.59 4.41 3 3 4.41 7.59 9 3 13.59 4.41 15
                            9 10.41 13.59 15 15 13.59 10.41 9 15 4.41z" />
                </svg>
            </a>

            <div class="flex--item fc-black-300 ml12 sm:d-none">
                <Sign />
            </div>

        </div>
    }
}

#[function_component(NavOver)]
fn nav_over() -> Html {
    html! {
        <div class="js-navigation">
            <div class="overflow-y-hidden overflow-x-hidden bg-white z-nav py8">

                <div class="d-none sm:d-block">
                    <div class="d-flex fd-column ai-center">

                        <nav class="s-navigation s-navigation__wrap mx8 mb12 jc-center"
                            aria-label="Global navigation">
                            <NavGlobal />
                        </nav>

                        <div class="d-flex gs16 ai-center fw-wrap mx8">

                            <button class="s-btn__unset c-pointer flex--item fc-black-300 js-darkmode-btn"
                                title={ CFG.get("theme_mode.title").unwrap().clone() }>
                                <svg aria-hidden="true" class="svg-icon iconTheme"
                                    width="24" height="24" viewBox="0 0 18 18">
                                    <path d={ CFG.get("theme_mode.svg").unwrap().clone() } />
                                </svg>
                            </button>

                            <a class="flex--item fc-black-300"
                                title={ CFG.get("i18n.title").unwrap().clone() }
                                href={ CFG.get("i18n.href").unwrap().clone() }>
                                <svg aria-hidden="true" class="svg-icon iconInternational"
                                    width="24" height="24" viewBox="0 0 24 24">
                                    <path d={ CFG.get("i18n.svg").unwrap().clone() } />
                                </svg>
                            </a>

                            <a class="flex--item fc-black-300"
                                title={ CFG.get("github.title").unwrap().clone() }
                                href={ CFG.get("github.href").unwrap().clone() } target="_blank">
                                <svg aria-hidden="true" class="svg-icon iconGitHub"
                                    width="24" height="24" viewBox="0 0 18 18">
                                    <path d={ CFG.get("github.svg").unwrap().clone() } />
                                </svg>
                            </a>

                            <div class="flex--item fc-black-300">
                                <Sign />
                            </div>

                        </div>

                    </div>
                </div>

            </div>
        </div>
    }
}
