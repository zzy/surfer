use yew::prelude::*;
use yew_functional::*;
use yew_router::prelude::*;

use crate::util::constant::CFG;
use crate::router::nav_routes::NavRoutes;

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
            // TODO: "s-navigation--item", "is-selected"
            <Link<NavRoutes> classes=classes!("s-navigation--item")
                route=NavRoutes::Articles>
                { "Articles" }
            </Link<NavRoutes>>
            <Link<NavRoutes> classes=classes!("s-navigation--item") route=NavRoutes::Categories>
                { "Categories" }
            </Link<NavRoutes>>
            <Link<NavRoutes> classes=classes!("s-navigation--item") route=NavRoutes::Topics>
                { "Topics" }
            </Link<NavRoutes>>
            <a class="s-navigation--item" href="//budshome.com/books.html"
                target="_blank">
                { "Explore" }
            </a>
        </>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <div class="grid ai-center px16 h100 mx-auto wmx12 sm:jc-space-between">

            <a class="grid grid__center p8 mr8 s-link s-link__muted d-none md:d-block js-hamburger-btn"
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

            <Link<NavRoutes> classes=classes!("grid--cell", "js-logo") route=NavRoutes::Home>
                <span class="fs-title fw-bold">
                    <span class="ff-sans">{ "锈毂" }</span>
                    <img class="va-sub" src="/imgs/logos/rusthub.png"
                        alt={ "RustHub - 锈毂" } title={ "RustHub - 锈毂" }
                        aria-label={ "RustHub - 锈毂" } />
                    <span class="ff-serif">{ "RustHub" }</span>
                </span>
            </Link<NavRoutes>>

            <nav class="s-navigation mx16 fw-nowrap sm:d-none" aria-label="Global navigation">
                <NavGlobal />
            </nav>

            <button class="s-btn__unset c-pointer grid--cell fc-black-300 ml12 sm:d-none
                js-darkmode-btn ml-auto"
                title={ CFG.get("theme_mode.title").unwrap().to_owned() }>
                <svg aria-hidden="true" class="svg-icon iconTheme"
                    width="24" height="24" viewBox="0 0 18 18">
                    <path d={ CFG.get("theme_mode.svg").unwrap().to_owned() } />
                </svg>
            </button>

            <a class="grid--cell fc-black-300 ml12 sm:d-none"
                title={ CFG.get("i18n.title").unwrap().to_owned() }
                href={ CFG.get("i18n.href").unwrap().to_owned() }>
                <svg aria-hidden="true" class="svg-icon iconInternational"
                    width="24" height="24" viewBox="0 0 24 24">
                    <path d={ CFG.get("i18n.svg").unwrap().to_owned() } />
                </svg>
            </a>

            <a class="grid--cell fc-black-300 ml12 sm:d-none"
                title={ CFG.get("github.title").unwrap().to_owned() }
                href={ CFG.get("github.href").unwrap().to_owned() } target="_blank">
                <svg aria-hidden="true" class="svg-icon iconGitHub"
                    width="24" height="24" viewBox="0 0 18 18">
                    <path d={ CFG.get("github.svg").unwrap().to_owned() } />
                </svg>
            </a>

            <div class="grid--cell ps-relative ml16 w100 wmx3 sm:wmx-initial sm:ml0 sm:d-none js-search">
                <input id="searchbox" class="s-input s-input__search bar-md js-stacks-search-bar"
                    type="text" placeholder={ "Search RustHub …" } />
                <svg aria-hidden="true" class="svg-icon iconSearch s-input-icon s-input-icon__search"
                    width="18" height="18" viewBox="0 0 18 18">
                    <path
                        d="m18 16.5-5.14-5.18h-.35a7 7 0 10-1.19 1.19v.35L16.5 18l1.5-1.5zM12
                            7A5 5 0 112 7a5 5 0 0110 0z" />
                </svg>
            </div>

            <a class="grid grid__center p8 ml8 s-link s-link__muted d-none sm:d-block js-search-btn"
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

        </div>
    }
}

#[function_component(NavOver)]
fn nav_over() -> Html {
    html! {
        <div class="js-navigation">
            <div class="overflow-y-hidden overflow-x-hidden bg-white z-nav py8">

                <div class="d-none sm:d-block">
                    <div class="grid fd-column ai-center">

                        <nav class="s-navigation s-navigation__wrap mx8 mb12 jc-center"
                            aria-label="Global navigation">
                            <NavGlobal />
                        </nav>

                        <div class="grid gs16 ai-center fw-wrap mx8">

                            <button class="s-btn__unset c-pointer grid--cell fc-black-300 js-darkmode-btn"
                                title={ CFG.get("theme_mode.title").unwrap().to_owned() }>
                                <svg aria-hidden="true" class="svg-icon iconTheme"
                                    width="24" height="24" viewBox="0 0 18 18">
                                    <path d={ CFG.get("theme_mode.svg").unwrap().to_owned() } />
                                </svg>
                            </button>

                            <a class="grid--cell fc-black-300"
                                title={ CFG.get("i18n.title").unwrap().to_owned() }
                                href={ CFG.get("i18n.href").unwrap().to_owned() }>
                                <svg aria-hidden="true" class="svg-icon iconInternational"
                                    width="24" height="24" viewBox="0 0 24 24">
                                    <path d={ CFG.get("i18n.svg").unwrap().to_owned() } />
                                </svg>
                            </a>

                            <a class="grid--cell fc-black-300"
                                title={ CFG.get("github.title").unwrap().to_owned() }
                                href={ CFG.get("github.href").unwrap().to_owned() } target="_blank">
                                <svg aria-hidden="true" class="svg-icon iconGitHub"
                                    width="24" height="24" viewBox="0 0 18 18">
                                    <path d={ CFG.get("github.svg").unwrap().to_owned() } />
                                </svg>
                            </a>

                        </div>

                    </div>
                </div>

            </div>
        </div>
    }
}
