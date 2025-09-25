use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
mod components;
mod data;

use crate::components::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Menu,
    #[route("/editor")]
    Editor
}

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Rust Ressources Editor")
                    .with_resizable(true),
            ),
        )
        .launch(App)
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
