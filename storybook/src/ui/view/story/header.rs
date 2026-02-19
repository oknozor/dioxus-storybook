use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Breadcrumb header displayed at the top of a story page.
///
/// Renders the component name and story title separated by a `/` divider,
/// giving the user context about which story they are currently viewing.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `component_name` | `String` | Name of the component (left side). |
/// | `story_title` | `String` | Title of the active story (right side). |
///
/// @[story:Molecules/StoryHeader/Default]
///
/// @[story:Molecules/StoryHeader/Long Names]
#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn StoryHeader(component_name: String, story_title: String) -> Element {
    rsx! {
        div { class: "story-page-header",
            span { class: "story-page-component-name", "{component_name}" }
            span { class: "story-page-separator", "/" }
            span { class: "story-page-story-name", "{story_title}" }
        }
    }
}
