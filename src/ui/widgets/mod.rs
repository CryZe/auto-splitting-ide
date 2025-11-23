use dioxus::prelude::*;

mod auto_splitter_control;
mod logs;
mod processes;
mod settings_gui;
mod settings_map;
mod statistics;
mod timer_info;

pub use auto_splitter_control::*;
pub use logs::*;
pub use processes::*;
pub use settings_gui::*;
pub use settings_map::*;
pub use statistics::*;
pub use timer_info::*;

#[component]
pub fn Widget(title: &'static str, children: Element) -> Element {
    let mut visible = use_signal(|| true);
    let mut max_height = use_signal(|| 0.0);

    rsx! {
        div { "data-swapy-slot": title,
            div { "data-swapy-item": title, class: "widget",
                h2 {
                    "data-swapy-handle": true,
                    cursor: "pointer",
                    position: "relative",
                    onclick: move |_| {
                        *visible.write() ^= true;
                    },
                    div {
                        position: "absolute",
                        transform: if *visible.read() { "rotate(0deg)" } else { "rotate(-90deg)" },
                        transition: "transform 0.25s",
                        svg {
                            width: "22px",
                            height: "22px",
                            view_box: "0 0 24 24",
                            path {
                                fill: "white",
                                d: "M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6z",
                            }
                        }
                    }
                    {title}
                }
                div {
                    class: "collapsible",
                    height: if *visible.read() { "auto" } else { "0" },
                    opacity: if *visible.read() { "1" } else { "0" },
                    onmounted: move |element| async move {
                        if let Ok(size) = element.data.get_scroll_size().await {
                            max_height.set(size.height);
                        }
                    },
                    {children}
                }
            }
        }
    }
}
