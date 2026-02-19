use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Tr(
    #[props(extends = GlobalAttributes, extends = tr)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tr { class: "prop-row editable", ..attributes, {children} }
    }
}

#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Td(
    #[props(extends = GlobalAttributes, extends = td)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        td { class: "prop-cell", ..attributes, {children} }
    }
}
