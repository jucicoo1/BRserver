#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/radar/")]
    Radar {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

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
    // let mut count = use_signal(|| 0);
    rsx! {
        div {
            id: "PAGE-CONTAINER",
            div {
                class: "topnav",
                id: "myTopnav",
                a {
                    class: "active",
                    href: "#home",
                    "Home"
                }
                a {
                    href: "#blog",
                    "Blog"
                }
                a {
                    href: "#Radar",
                    "Radar"
                }
                a {
                    class: "icon",
                    href: "javascript:void(0);",
                    "Radar",
                    i{
                        class: "fa fa-bars",
                    }
                }
            }
            div {
                id: "home-background",
                div {
                    class: "middle-content",
                    // div {
                    //     class: "middle-left-content",
                    // }
                    // div {
                    //     class: "middle-right-content",
                        
                    // }
                    h1 { 
                        class: "home_h1",
                        "In the grim darkness"
                    }
                    h1 { 
                        class: "home_h1",
                        "of the far future,"
                    }
                    h1 { 
                        class: "home_h1",
                        "there is only war"
                    }
                    button {
                        id: "to_battle",
                        "Report Your Battle!"
                    }
                    button {
                        id: "to_enroll",
                        "Enroll Your Army!"
                    }
                }
            }
        }
    }
}


#[component]
fn Radar() -> Element {
    rsx! {
        Link {
            to: Route::Home {},
            "Go to Home"
        }
        div {
            h1 {"this is going to be radar page"}
        }
    }
}
