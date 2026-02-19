use crate::{RenderFn, StoryInfo, find_component};
use schemars::Schema;

/// Resolved data for a StoryPage view.
///
/// The viewmodel calls `find_component()` and extracts everything the view
/// needs so the view never touches the data-access layer directly.
pub struct StoryPageData {
    pub story: StoryInfo,
    pub story_title: String,
    pub render_fn: RenderFn,
    pub prop_schema: Schema,
}

/// Error cases when resolving a story page.
pub enum StoryPageError {
    ComponentNotFound(String),
    StoryNotFound {
        component_name: String,
        story_index: usize,
    },
}

/// Look up a component by name and resolve the story at `story_index`.
///
/// Returns the fully resolved [`StoryPageData`] or a [`StoryPageError`]
/// describing what went wrong.
pub fn resolve_story_page(
    component_name: &str,
    story_index: usize,
) -> Result<StoryPageData, StoryPageError> {
    let registration = find_component(component_name)
        .ok_or_else(|| StoryPageError::ComponentNotFound(component_name.to_string()))?;

    let stories = (registration.get_stories)();
    let render_fn = registration.render_with_props;
    let prop_schema = (registration.get_prop_schema)();

    let story = stories
        .get(story_index)
        .cloned()
        .ok_or_else(|| StoryPageError::StoryNotFound {
            component_name: component_name.to_string(),
            story_index,
        })?;

    let story_title = story.title.clone();

    Ok(StoryPageData {
        story,
        story_title,
        render_fn,
        prop_schema,
    })
}
