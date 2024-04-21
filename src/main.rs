#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use wasm_bindgen::prelude::*;

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
            menu_bar {},
            div {
                id: "home-background",
                div {
                    class: "middle-content",
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
        menu_bar {},
        div {
            h1 {"this is going to be radar page"}
        }
    }
}

#[component]
fn menu_bar() -> Element {
    rsx! {
        div {
            class: "topnav",
            id: "myTopnav",
            a {
                class: "active",
                href: "/",
                "Home"
            }
            a {
                href: "#blog",
                "Blog"
            }
            a {
                href: "/radar/",
                "Radar"
            }
            a {
                class: "icon",
                href: "javascript:void(0);",
                onclick: move |_| {
                    // Call the JavaScript function through the binding
                    call_test();
                },
                "Icon",
                i{
                    class: "fa fa-bars",
                }
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn call_test();
}