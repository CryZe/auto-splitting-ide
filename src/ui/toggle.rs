use dioxus::prelude::*;

#[component]
pub fn Toggle(
    children: Element,
    checked: bool,
    #[props(default)] centered: bool,
    onchange: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        label {
            display: "flex",
            gap: "8px",
            cursor: "pointer",
            justify_content: if centered { "center" },
            div { class: "switch",
                input { r#type: "checkbox", checked, onchange }
                span {}
            }
            {children}
        }
    }
}
