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
            link{rel:"stylesheet",href:"website.min.css"}
            main { class: "bd-docs is-fullwidth is-active", id: "js-docs",
                button {
                    "data-target": "js-docs",
                    class: "button bd-docs-button js-toggle is-active",
                    id: "js-menu-toggle",
                    i { class: "fas fa-arrow-right-from-bracket" }
                }
                div { class: "bd-docs-menu",
                    div { class: "bd-menu",
                        div { class: "bd-menu-section is-start",
                            h3 { class: "bd-menu-heading js-menu-heading",
                                span { class: "icon",
                                    i { class: "fas fa-arrow-right" }
                                }
                                strong { "Start Here" }
                            }
                            ul {
                                style: "--size: 7",
                                class: "bd-menu-list js-menu-list",
                                li {
                                    a {
                                        "data-current-link-id": "components-navbar",
                                        href: "https://bulma.io/documentation/start/migrating-to-v1/",
                                        "data-size": "0",
                                        "data-name": "Migrating to v1",
                                        class: "bd-menu-link",
                                        span { class: "bd-menu-link-name", "Migrating to v1" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bd-side-sponsrs",
                        p { class: "bd-side-sponsor-label", "Sponsor" }
                        a {
                            href: "https://www.route4me.com/",
                            target: "_blank",
                            class: "bd-side-sponsor",
                            img {
                                src: "https://bulma.io/assets/images/amis/route4me.png",
                                width: "224",
                                alt: "Route Planner and Route Optimizer",
                                srcset: "https://bulma.io/assets/images/amis/route4me.png 1x,\n          https://bulma.io/assets/images/amis/route4me@2x.png 2x",
                                height: "57"
                            }
                        }
                    }
                }
                div { class: "bd-docs-lead bd-theme-library",
                    nav { "aria-label": "breadcrumbs", class: "breadcrumb is-right",
                        ul {
                            li {
                                a { href: "#", "Bulma" }
                            }
                            li {
                                a { href: "#", "Documentation" }
                            }
                            li {
                                a { href: "#", "Components" }
                            }
                            li { class: "is-active",
                                a { href: "#", "aria-current": "page", "Breadcrumb" }
                            }
                        }
                    }
                    // div { class: "container ",
                        table { class: "table",
                            thead {
                                tr {
                                    th {
                                        abbr { title: "Position", "Pos" }
                                    }
                                    th { "Team" }
                                    th {
                                        abbr { title: "Played", "Pld" }
                                    }
                                    th {
                                        abbr { title: "Won", "W" }
                                    }
                                    th {
                                        abbr { title: "Drawn", "D" }
                                    }
                                    th {
                                        abbr { title: "Lost", "L" }
                                    }
                                    th {
                                        abbr { title: "Goals for", "GF" }
                                    }
                                    th {
                                        abbr { title: "Goals against", "GA" }
                                    }
                                    th {
                                        abbr { title: "Goal difference", "GD" }
                                    }
                                    th {
                                        abbr { title: "Points", "Pts" }
                                    }
                                    th { "Qualification or relegation" }
                                }
                            }
                            tfoot {
                                tr {
                                    th {
                                        abbr { title: "Position", "Pos" }
                                    }
                                    th { "Team" }
                                    th {
                                        abbr { title: "Played", "Pld" }
                                    }
                                    th {
                                        abbr { title: "Won", "W" }
                                    }
                                    th {
                                        abbr { title: "Drawn", "D" }
                                    }
                                    th {
                                        abbr { title: "Lost", "L" }
                                    }
                                    th {
                                        abbr { title: "Goals for", "GF" }
                                    }
                                    th {
                                        abbr { title: "Goals against", "GA" }
                                    }
                                    th {
                                        abbr { title: "Goal difference", "GD" }
                                    }
                                    th {
                                        abbr { title: "Points", "Pts" }
                                    }
                                    th { "Qualification or relegation" }
                                }
                            }
                            tbody {
                                tr {
                                    th { "1" }
                                    td {
                                        a {
                                            title: "Leicester City F.C.",
                                            href: "https://en.wikipedia.org/wiki/Leicester_City_F.C.",
                                            "Leicester City"
                                        }
                                        strong { "(C)" }
                                    }
                                    td { "38" }
                                    td { "23" }
                                    td { "12" }
                                    td { "3" }
                                    td { "68" }
                                    td { "36" }
                                    td { "+32" }
                                    td { "81" }
                                    td {
                                        "\n                    Qualification for the\n                    "
                                        a {
                                            href: "https://en.wikipedia.org/wiki/2016%E2%80%9317_UEFA_Champions_League#Group_stage",
                                            title: "2016–17 UEFA Champions League",
                                            "Champions League group stage"
                                        }
                                    }
                                }
                            }
                        }
                    // }
                }
            }
    }

}
