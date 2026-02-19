use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn StoryDocs(docs: String) -> Element {
    rsx! {
        div { class: "component-description", dangerous_inner_html: "{docs}" }
    }
}
