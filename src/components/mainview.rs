use std::process::exit;

use dioxus::prelude::*;
use crate::components::{Button};
use rfd::FileDialog;

#[component]
pub fn MainView() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center h-screen gap-4 max-w-xs md:max-w-xl xl:max-w-2xl mx-auto",
            Button {  
                "data-style": "primary",
                onclick: |_| {
                      if let Some(path) = FileDialog::new()
                          .add_filter("Config Files", &["yaml", "yml", "json"])
                          .pick_file() {
                            println!("Selected file: {:?}", path);
                        } else {
                            println!("No file selected");
                      }
                },
                "Open File" 
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
