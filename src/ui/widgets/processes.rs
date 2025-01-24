use dioxus::prelude::*;

use super::Widget;

#[component]
pub fn Processes(processes: SyncSignal<Vec<(String, String)>>) -> Element {
    rsx! {
        Widget { title: "Processes",
            if !processes.read().is_empty() {
                div { class: "table",
                    for (pid , path) in processes.read().iter() {
                        div { "{pid}" }
                        div { "{path}" }
                    }
                }
            }
        }
    }
}
