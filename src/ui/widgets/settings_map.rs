use dioxus::prelude::*;
use livesplit_auto_splitting::{
    settings::{self, Value},
    AutoSplitter,
};

use crate::{
    ui::{Panel, Widget},
    IdeTimer,
};

#[component]
pub fn SettingsMap(
    settings_map: SyncSignal<settings::Map>,
    auto_splitter: SyncSignal<Option<AutoSplitter<IdeTimer>>>,
) -> Element {
    rsx! {
        Widget { title: "Settings Map",
            if !settings_map.read().is_empty() {
                div { class: "table",
                    for (key , value) in settings_map.read().iter() {
                        div { "{key}" }
                        div { class: "setting-value", {show_value(value)} }
                    }
                }
                button {
                    onclick: move |_| {
                        if let Some(auto_splitter) = auto_splitter.read().as_ref() {
                            auto_splitter.set_settings_map(settings::Map::new());
                            settings_map.set(auto_splitter.settings_map());
                        }
                    },
                    "Clear"
                }
            }
        }
    }
}

fn show_value(value: &Value) -> Element {
    match value {
        Value::Map(map) => rsx! {
            div { class: "table", title: "map",
                for (key , value) in map.iter() {
                    div { "{key}" }
                    div { class: "setting-value", {show_value(value)} }
                }
            }
        },
        Value::List(list) => rsx! {
            Panel { title: "list",
                for (i , value) in list.iter().enumerate() {
                    if i != 0 {
                        hr {}
                    }
                    div { class: "setting-value", {show_value(value)} }
                }
            }
        },
        Value::Bool(value) => rsx! {
            span {
                class: if *value { "green" } else { "red" },
                font_weight: "bold",
                title: "bool",
                if *value {
                    "true"
                } else {
                    "false"
                }
            }
        },
        Value::I64(value) => rsx! {
            span { class: "blue", font_weight: "bold", title: "i64", "{value}" }
        },
        Value::F64(value) => rsx! {
            span { class: "pink", font_weight: "bold", title: "f64", "{value}" }
        },
        Value::String(value) => rsx! {
            span { class: "yellow", font_weight: "bold", title: "string", "{value:?}" }
        },
        _ => rsx! {
            span { class: "gray", font_weight: "bold", "Unknown" }
        },
    }
}
