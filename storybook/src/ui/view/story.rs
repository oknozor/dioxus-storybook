use crate::ui::view::story::header::StoryHeader;
use crate::{RenderFn, StoryInfo};
use dioxus::prelude::*;
use schemars::Schema;

mod header;
mod preview;
mod toolbar;
pub use toolbar::StoryZoomControls;

#[cfg(feature = "self-stories")]
mod stories;

mod card;
use crate::ui::view::story::preview::StoryPreview;
pub use card::StoryCard;

pub mod props_editor;

/// A dedicated page for displaying a single story in full-screen mode.
///
/// Pure presentational component — all data resolution is handled by the parent.
#[component]
pub(crate) fn StoryPage(
    component_name: String,
    story_index: usize,
    story: StoryInfo,
    story_title: String,
    render_fn: RenderFn,
    prop_schema: Schema,
) -> Element {
    rsx! {
        div { class: "story-page",
            StoryHeader { component_name: component_name.clone(), story_title }

            StoryPreview {
                key: "{component_name}-{story_index}",
                story,
                component_name,
                story_index,
                render_fn,
                prop_schema,
            }
        }
    }
}
