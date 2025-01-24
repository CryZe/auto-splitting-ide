use dioxus::prelude::*;

pub struct DividerState {
    pub size: f64,
    pub is_dragging: bool,
    pub positive: bool,
    pub vertical: bool,
    pub min: f64,
    pub drag_start: f64,
}

#[component]
pub fn VerticalDivider(state: Signal<DividerState>) -> Element {
    rsx! {
        div {
            height: "100%",
            margin: "0 -7px",
            cursor: "ew-resize",
            onmousedown: move |ev| {
                ev.prevent_default();
                let state = &mut *state.write();
                let mouse_coords = ev.client_coordinates();
                let coord = if state.vertical { mouse_coords.y } else { mouse_coords.x };
                state.drag_start = coord;
                state.is_dragging = true;
            },
            div {
                class: "divider vertical",
                class: if state.read().is_dragging { "dragging" },
            }
        }
    }
}

#[component]
pub fn HorizonalDivider(state: Signal<DividerState>) -> Element {
    rsx! {
        div {
            width: "100%",
            margin: "-7px 0",
            cursor: "ns-resize",
            onmousedown: move |ev| {
                ev.prevent_default();
                let state = &mut *state.write();
                let mouse_coords = ev.client_coordinates();
                let coord = if state.vertical { mouse_coords.y } else { mouse_coords.x };
                state.drag_start = coord;
                state.is_dragging = true;
            },
            div {
                class: "divider horizontal",
                class: if state.read().is_dragging { "dragging" },
            }
        }
    }
}
