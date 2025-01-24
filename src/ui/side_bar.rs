use dioxus::prelude::*;

#[component]
pub fn SideBar(children: Element, drag_side: bool, width: f64) -> Element {
    rsx! {
        div {
            class: "bar",
            display: "flex",
            flex_direction: "column",
            gap: "10px",
            height: "100%",
            flex: "0 0 {width}px",
            max_width: "{width}px",
            {children}
        }
    }
}
