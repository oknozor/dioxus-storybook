use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

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
