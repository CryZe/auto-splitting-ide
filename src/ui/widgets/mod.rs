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
        h2 {
            cursor: "pointer",
            position: "relative",
            onclick: move |_| {
                *visible.write() ^= true;
            },
            div {
                position: "absolute",
                transform: if *visible.read() { "rotate(0deg)" } else { "rotate(-90deg)" },
                transition: "transform 0.25s",
                img {
                    width: "22px",
                    height: "22px",
                    src: asset!("assets/collapse.svg"),
                }
            }
            {title}
        }
        div {
            class: "collapsible",
            display: if *visible.read() { "contents" } else { "none" },
            onmounted: move |element| async move {
                if let Ok(size) = element.data.get_scroll_size().await {
                    max_height.set(size.height);
                }
            },
            {children}
        }
    }
}
