use dioxus::prelude::*;
use schemars::schema::RootSchema;
use crate::StoryInfo;
use crate::ui::view::story::docs::StoryDocs;
use crate::ui::view::story::header::StoryHeader;

mod docs;
mod header;
mod preview;
mod toolbar;
pub use toolbar::StoryZoomControls;

mod card;
pub use card::StoryCard;
use crate::ui::view::story::preview::StoryPreview;

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
    render_fn: fn(&str) -> Element,
    prop_schema: RootSchema,
    description: &'static str,
) -> Element {
    rsx! {
        div { class: "story-page",
            StoryHeader { component_name: component_name.clone(), story_title }

            if !description.is_empty() {
                StoryDocs { docs: description }
            }

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

