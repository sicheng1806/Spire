#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn run() {
    launch(App);
}

/// The main app
pub fn App() -> Element {
    let buttons: &str = r#"<button class="btn">Button</button>
<button class="btn btn-neutral">Neutral</button>
<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary">Secondary</button>
<button class="btn btn-accent">Accent</button>
<button class="btn btn-ghost">Ghost</button>
<button class="btn btn-link">Link</button>"#;

    rsx! {
       div {
           dangerous_inner_html: buttons
       }
    }
}
