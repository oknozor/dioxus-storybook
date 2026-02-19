use crate::{RenderWithPropsFn, StoryInfo, find_component};
use schemars::schema::RootSchema;

/// Resolved data for an EmbeddedStory view.
///
/// The viewmodel parses the story path, calls `find_component()`, and
/// locates the matching story so the view only deals with ready-to-render data.
pub struct EmbeddedStoryData {
    pub component_name: String,
    pub story_index: usize,
    pub story: StoryInfo,
    pub render_fn: RenderWithPropsFn,
    pub prop_schema: RootSchema,
}

/// Error cases when resolving an embedded story.
pub enum EmbeddedStoryError {
    InvalidPath(String),
    ComponentNotFound(String),
    StoryNotFound { component_name: String, story_name: String },
}

/// Parse a story path and resolve the component + story.
///
/// The `story_path` format is `"Category/Component/StoryName"` (at least two
/// segments). The component name is the second-to-last segment.
///
/// Returns the fully resolved [`EmbeddedStoryData`] or an
/// [`EmbeddedStoryError`] describing what went wrong.
pub fn resolve_embedded_story(
    story_path: &str,
    story_name: &str,
) -> Result<EmbeddedStoryData, EmbeddedStoryError> {
    let path_parts: Vec<&str> = story_path.split('/').collect();

    if path_parts.len() < 2 {
        return Err(EmbeddedStoryError::InvalidPath(story_path.to_string()));
    }

    let component_name = path_parts[path_parts.len() - 2];

    let registration = find_component(component_name).ok_or_else(|| {
        EmbeddedStoryError::ComponentNotFound(component_name.to_string())
    })?;

    let stories = (registration.get_stories)();
    let story_with_index = stories
        .iter()
        .enumerate()
        .find(|(_, s)| s.title == story_name);

    let (story_index, story) = story_with_index.ok_or_else(|| {
        EmbeddedStoryError::StoryNotFound {
            component_name: component_name.to_string(),
            story_name: story_name.to_string(),
        }
    })?;

    let render_fn = registration.render_with_props;
    let prop_schema = (registration.get_prop_schema)();

    Ok(EmbeddedStoryData {
        component_name: component_name.to_string(),
        story_index,
        story: story.clone(),
        render_fn,
        prop_schema,
    })
}

