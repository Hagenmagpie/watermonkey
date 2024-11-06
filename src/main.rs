#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {

        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
            link{rel:"stylesheet",href:"bulma.css"}

            nav { class: "navbar is-transparent",
                div { class: "navbar-brand",
                    a { href: "https://www.watermonkey.org", class: "navbar-item",
                        img { src: "logo.png" }
                        b { "WATERMONKEY" }
                    }
                    div {
                        "data-target": "navbarExampleTransparentExample",
                        class: "navbar-burger js-burger",
                        span {}
                        span {}
                        span {}
                        span {}
                    }
                }
                div { class: "navbar-menu", id: "navbarExampleTransparentExample",
                    div { class: "navbar-start",
                        a { href: "https://www.watermonkey.org/", class: "navbar-item", " Home " }
                    }
                    div { class: "navbar-end",
                        div { class: "navbar-item",
                            div { class: "field is-grouped",
                                p { class: "control",
                                    a {
                                        "data-social-network": "Twitter",
                                        "data-social-action": "tweet",
                                        href: "https://twitter.com/intent/tweet?text=Bulma: a modern CSS framework based on Flexbox&amp;hashtags=bulmaio&amp;url=https://bulma.io&amp;via=jgthms",
                                        "data-social-target": "https://bulma.io",
                                        target: "_blank",
                                        class: "bd-tw-button button",
                                        span { class: "icon",
                                            i { class: "fab fa-twitter" }
                                        }
                                        span { " GITHUB " }
                                    }
                                }
                                p { class: "control",
                                    a {
                                        href: "https://github.com/jgthms/bulma/releases/download/1.0.2/bulma-1.0.2.zip",
                                        class: "button is-primary",
                                        span { class: "icon",
                                            i { class: "fas fa-download" }
                                        }
                                        span { "DOWNLOAD" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Link {
            //     to: Route::Blog {
            //         id: count()
            //     },
            //     "Go to blog"
            // }
            //
            // div {
            //     h1 { "High-Five counter: {count}" }
            //     button { class:"button is-warning", onclick: move |_| count += 1, "Up high!" }
            //     button { onclick: move |_| count -= 1, "Down low!" }
            // }

    }

}
