use dioxus::prelude::*;
use crate::ui::story::docs::StoryDocs;
use crate::ui::story::header::StoryHeader;
use crate::{find_component, Decorator};

mod docs;
mod header;
mod iframe;
mod preview;
mod toolbar;
pub use toolbar::StoryZoomControls;

mod card;
pub use card::StoryCard;
use crate::ui::story::preview::StoryPreview;

pub mod props_editor;

/// A dedicated page for displaying a single story in full-screen mode.
#[component]
pub(crate) fn StoryPage(
    component_name: String,
    story_index: usize,
    #[props(default)] attribute: Vec<Attribute>,
) -> Element {
    let Some(registration) = find_component(&component_name) else {
        return rsx! {
            div { class: "error", "Component not found: {component_name}" }
        };
    };

    let current_stories = (registration.get_stories)();
    let render_fn = registration.render_with_props;
    let prop_schema = (registration.get_prop_schema)();

    let Some(story) = current_stories.get(story_index) else {
        return rsx! {
            div { class: "error", "Story not found: index {story_index} for {component_name}" }
        };
    };

    rsx! {
        div { class: "story-page",
            StoryHeader {
                component_name: component_name.clone(),
                story_title: story.title.clone(),
            }

            if !registration.description.is_empty() {
                StoryDocs { docs: registration.description }
            }

            StoryPreview {
                key: "{component_name}-{story_index}",
                story: story.clone(),
                component_name,
                story_index,
                render_fn,
                prop_schema,
            }
        }
    }
}

/// Apply decorators to an element.
/// Decorators are applied in order, with the first decorator being the outermost wrapper.
fn apply_decorators(element: Element, decorators: &[Decorator]) -> Element {
    decorators
        .iter()
        .rev()
        .fold(element, |acc, decorator| decorator(acc))
}


