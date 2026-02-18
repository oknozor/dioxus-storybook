use dioxus::prelude::*;

#[component]
pub fn Tr(
    #[props(extends = GlobalAttributes, extends = tr)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tr { class: "prop-row editable", ..attributes, {children} }
    }
}

#[component]
pub fn Td(
    #[props(extends = GlobalAttributes, extends = td)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        td { class: "prop-cell", ..attributes, {children} }
    }
}