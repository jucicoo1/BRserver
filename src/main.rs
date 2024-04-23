#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;
use wasm_bindgen::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/battle/")]
    BattleList {},
    #[route("/battle/new")]
    BattleNew {},
    #[route("/battle/edit/:id")]
    BattleEdit { id: i32 },
    #[route("/radar/:id")]
    Radar { id: i32 },
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
fn BattleList() -> Element {
    rsx! {
        menu_bar {},
        div {
            h1 {"this is going to be battle list page"}
        }
    }
}

#[component]
fn BattleNew() -> Element {
    rsx! {
        menu_bar {},
        div {
            h1 {"this is going to be battle new page"}
        }
    }
}

#[component]
fn BattleEdit(id: i32) -> Element {
    rsx! {
        menu_bar {},
        div {
            h1 {"this is going to be battle edit page"}
        }
    }
}

#[component]
fn Radar(id: i32) -> Element {
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