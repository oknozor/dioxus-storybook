use crate::ui::view::shared::{ResetZoomButton, ZoomInButton, ZoomOutButton};
use dioxus::prelude::*;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Zoom controls toolbar for the story preview.
///
/// Groups a `ZoomOutButton`, a percentage label, a `ZoomInButton`, and a
/// `ResetZoomButton` into a horizontal toolbar. The zoom range is 25 %–200 %
/// in 25 % increments.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `zoom_level` | `Signal<i32>` | Current zoom percentage shared with the preview. |
///
/// @[story:Molecules/StoryZoomControls/Default (100%)]
///
/// @[story:Molecules/StoryZoomControls/Zoomed In (150%)]
///
/// @[story:Molecules/StoryZoomControls/Zoomed Out (50%)]
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
