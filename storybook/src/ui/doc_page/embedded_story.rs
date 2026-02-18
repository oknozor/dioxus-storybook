use dioxus::prelude::*;
use crate::find_component;
use crate::ui::story::StoryCard;

/// Component to render an embedded story within a doc page
#[component]
pub fn EmbeddedStory(story_path: String, story_name: String) -> Element {
    // Parse the story path: "Category/Component/StoryName"
    let path_parts: Vec<&str> = story_path.split('/').collect();

    if path_parts.len() < 2 {
        return rsx! {
            div { class: "embedded-story-error", "Invalid story path: {story_path}" }
        };
    }

    // The component name is the second-to-last part, story name is the last
    let component_name = path_parts[path_parts.len() - 2];

    let Some(registration) = find_component(component_name) else {
        return rsx! {
            div { class: "embedded-story-error", "Component not found: {component_name}" }
        };
    };

    // Find the specific story and its index
    let stories = (registration.get_stories)();
    let story_with_index = stories
        .iter()
        .enumerate()
        .find(|(_, s)| s.title == story_name);

    let Some((story_index, story)) = story_with_index else {
        return rsx! {
            div { class: "embedded-story-error", "Story not found: {story_name} in {component_name}" }
        };
    };

    let render_fn = registration.render_with_props;
    let prop_schema = (registration.get_prop_schema)();

    rsx! {
        div { class: "embedded-story",
            StoryCard {
                story: story.clone(),
                component_name: component_name.to_string(),
                story_index,
                render_fn,
                prop_schema,
            }
        }
    }
}
