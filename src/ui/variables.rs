use dioxus::prelude::*;
use indexmap::IndexMap;

use super::Widget;

#[component]
pub fn Variables(variables: SyncSignal<IndexMap<String, String>>) -> Element {
    rsx! {
        Widget { title: "Variables",
            if !variables.read().is_empty() {
                div { class: "table",
                    for (key , value) in variables.read().iter() {
                        div { "{key}" }
                        div { "{value}" }
                    }
                }
            }
        }
    }
}
