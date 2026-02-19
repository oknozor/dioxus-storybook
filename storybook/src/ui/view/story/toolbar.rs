use crate::ui::view::shared::{ResetZoomButton, ZoomInButton, ZoomOutButton};
use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn StoryZoomControls(zoom_level: Signal<i32>) -> Element {
    rsx! {
        div { class: "story-toolbar",
            ZoomOutButton { zoom_level }
            span { class: "zoom-level", "{zoom_level()}%" }
            ZoomInButton { zoom_level }
            ResetZoomButton { zoom_level }
        }
    }
}
