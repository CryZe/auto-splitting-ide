use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_material_icons::MaterialIcon;
use livesplit_auto_splitting::LogLevel;

use crate::UTC_OFFSET;

use crate::ui::{FmtTime, Panel};

pub struct LogEntries {
    entries: Vec<LogEntry>,
}

impl LogEntries {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn push_level(&mut self, message: String, level: LogLevel) {
        self.entries
            .push(LogEntry::new(message, LogType::Runtime(level)));
    }

    pub fn push(&mut self, message: String) {
        self.entries
            .push(LogEntry::new(message, LogType::AutoSplitter));
    }
}

pub struct LogEntry {
    time: time::OffsetDateTime,
    message: String,
    ty: LogType,
}

impl LogEntry {
    fn new(message: String, ty: LogType) -> Self {
        Self {
            time: time::OffsetDateTime::now_utc().to_offset(*UTC_OFFSET.get().unwrap()),
            message,
            ty,
        }
    }
}

enum LogType {
    Runtime(LogLevel),
    AutoSplitter,
}

#[component]
pub fn Logs(logs: SyncSignal<LogEntries>, show_editor: Signal<bool>, height: f64) -> Element {
    let mut element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_memo(move || {
        if let Some(element) = &*element.read() {
            drop(element.scroll_to(ScrollBehavior::Smooth));
        }
    });

    let current_logs = &*logs.read();

    rsx! {
        div {
            flex_grow: if *show_editor.read() { "0" } else { "1" },
            height: if *show_editor.read() { "{height}px" },
            position: "relative",
            display: "flex",
            flex_direction: "column",
            overflow: "hidden",
            Panel {
                class: "logs",
                min_height: "20px",
                display: "flex",
                flex_direction: "column",
                gap: "5px",
                flex_grow: "1",
                for (i , entry) in current_logs.entries.iter().enumerate() {
                    if i != 0 {
                        hr { margin: "0" }
                    }
                    div {
                        onmounted: move |cx| {
                            if i + 1 == logs.read().entries.len() {
                                element.set(Some(cx.data()));
                            }
                        },
                        span {
                            color: "rgba(255, 255, 255, 0.5)",
                            padding_right: "10px",
                            "{FmtTime(entry.time)}"
                        }
                        span {
                            class: match entry.ty {
                                LogType::AutoSplitter => "",
                                LogType::Runtime(LogLevel::Error) => "error",
                                LogType::Runtime(LogLevel::Warning) => "warn",
                                _ => "info",
                            },
                            "{entry.message}"
                        }
                    }
                }
            }
            if !current_logs.entries.is_empty() {
                button {
                    title: "Clear logs",
                    class: "log-button",
                    position: "absolute",
                    bottom: "20px",
                    right: "20px",
                    width: "32px",
                    height: "32px",
                    onclick: move |_| {
                        logs.write().entries.clear();
                    },
                    MaterialIcon { name: "delete_outline", size: 20 }
                }
            }
        }
    }
}
