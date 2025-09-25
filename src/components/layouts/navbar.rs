use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "title",
            Link { to: Route::MainView,
                h1 { "Editor" }
            }
        }
        Outlet::<Route> {}
    }
}