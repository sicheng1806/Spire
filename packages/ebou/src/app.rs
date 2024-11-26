#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn run() {
    launch(App);
}

/// The main app
pub fn App() -> Element {
    rsx! { "Hello World!" }
}
