use std::process::exit;

use dioxus::prelude::*;
use rfd::FileDialog;

use crate::components::Button;
use crate::data::*;

pub static CURRENT_EDITOR_CONTEXT: Global<Signal<EditorData>, EditorData> = Signal::global(|| EditorData::default());

#[component]
pub fn Menu() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center h-screen max-w-xs md:max-w-xl xl:max-w-2xl mx-auto",
            div {
                class: "flex flex-col gap-4",
                if !CURRENT_EDITOR_CONTEXT.read().name.is_empty() {
                    Button {
                        "data-style": "primary",
                        onclick: move |_| {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Config Files", &["yaml", "yml", "json"])
                                .pick_file() {
                                    match load_editor(&path) {
                                        Ok(data) => {
                                            println!("Path buffer: {:?}", path);
                                            *CURRENT_EDITOR_CONTEXT.write() = data;
                                            router().push("/editor");
                                        }
                                        Err(err) => {
                                            eprintln!("Failed to load config: {}", err);
                                        }
                                    }
                                    println!("Selected file: {:?}", path);
                                } else {
                                    println!("No file selected");
                            }
                        },
                        "Open File"
                    }
                }
                Button {
                    "data-style": "outline",
                    onclick: |_| {
                        println!("Quit button clicked");
                        exit(0);
                        #[allow(unreachable_code)]
                        ()
                    },
                    "Quit"
                }
            }
        }
    }
}
