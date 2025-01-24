use dioxus::prelude::*;

#[component]
pub fn Panel(
    children: Element,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {
            border_radius: "10px",
            padding: "10px",
            background: "#ffffff10",
            ..attributes,
            {children}
        }
    }
}
