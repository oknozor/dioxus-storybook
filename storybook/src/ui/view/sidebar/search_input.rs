use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;


#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn SearchInput(search_query: Signal<String>) -> Element {
    rsx! {
        div { class: "search-container",
            input {
                class: "search-input",
                r#type: "text",
                placeholder: "Search components...",
                value: "{search_query}",
                oninput: move |e| search_query.set(e.value()),
            }
        }
    }
}
