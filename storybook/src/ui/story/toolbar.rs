use dioxus::prelude::*;
use crate::ui::shared::{ResetZoomButton, ZoomInButton, ZoomOutButton};

#[component]
pub fn StoryZoomControls(zoom_level: Signal<i32>     ) -> Element {
    rsx! {
        div { class: "story-toolbar",
            ZoomOutButton { zoom_level }
            span { class: "zoom-level", "{zoom_level()}%" }
            ZoomInButton { zoom_level }
            ResetZoomButton { zoom_level }
        }
    }
}
