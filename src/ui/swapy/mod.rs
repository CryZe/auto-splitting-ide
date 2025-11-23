use dioxus::{document::eval, prelude::*};

#[component]
pub fn Container(
    id: &'static str,
    #[props(extends = GlobalAttributes, extends = div)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            id,
            onmounted: move |_| {
                let eval = eval(concat!(include_str!("swapy.js"), include_str!("use_swapy.js")));
                eval.send(id).unwrap();
            },
            ..attributes,
            {children}
        }
    }
}
