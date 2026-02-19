use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Search input for filtering components in the sidebar.
///
/// Renders a text input with a "Search components…" placeholder. The
/// `search_query` signal is updated on every keystroke, and the sidebar
/// tree filters components whose names match the query.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `search_query` | `Signal<String>` | Two-way bound search string. |
///
/// @[story:Molecules/SearchInput/Empty]
///
/// @[story:Molecules/SearchInput/With Query]
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
