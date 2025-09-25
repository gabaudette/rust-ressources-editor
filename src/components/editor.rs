use dioxus::prelude::*;

use crate::components::CURRENT_EDITOR_CONTEXT;



#[component]
pub fn Editor() -> Element {
    rsx! {
        div { class: "w-full h-full flex flex-col",
            div { class: "w-full h-12 bg-gray-800 flex items-center px-4",
                h1 { class: "text-white text-lg font-bold", {
                    CURRENT_EDITOR_CONTEXT.read().name.clone()
                } }
            }
            div { class: "flex-1 flex",
                div { class: "w-64 bg-gray-700 p-4",
                    h2 { class: "text-white text-md font-semibold mb-4", "Ressources" }
                }
                div { class: "flex-1 bg-gray-600 p-4",
                    h2 { class: "text-white text-md font-semibold mb-4", "Details" }
                    // Details view goes here
                }
            }
        }
    }
}