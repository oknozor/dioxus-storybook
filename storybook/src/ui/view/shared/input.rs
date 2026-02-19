use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn Checkbox(
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        input {
            class: "prop-input prop-checkbox",
            r#type: "checkbox",
            onchange: move |e| onchange.call(e.checked()),
            ..attributes,
        }
    }
}

#[cfg_attr(feature = "self-stories", storybook(tag = "Atoms"))]
#[component]
pub fn TextInput(
    oninput: EventHandler<String>,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        input {
            class: "prop-input",
            oninput: move |e| oninput.call(e.value()),
            ..attributes,
        }
    }
}
