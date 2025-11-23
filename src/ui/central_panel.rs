use dioxus::prelude::*;

use super::{DividerState, LogEntries, Logs};

#[component]
pub fn CentralPanel(logs: SyncSignal<LogEntries>, bottom_divider: Signal<DividerState>) -> Element {
    let show_editor = use_signal(|| false);
    rsx! {
        div {
            class: "bar",
            overflow_y: "auto",
            flex_grow: "1",
            display: "flex",
            flex_direction: "column",
            gap: "10px",
            Logs {
                logs,
                show_editor,
                height: bottom_divider.read().size,
            }
        }
    }
}
