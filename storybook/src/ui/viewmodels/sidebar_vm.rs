use crate::find_component;

/// Look up the story titles for a given component name.
///
/// Returns the list of story titles (e.g. `["Default", "Loading"]`) by
/// calling `find_component()` and extracting the title from each story.
/// Returns an empty `Vec` if the component is not found.
pub fn get_story_titles(component_name: &str) -> Vec<String> {
    find_component(component_name)
        .map(|reg| (reg.get_stories)().into_iter().map(|s| s.title).collect())
        .unwrap_or_default()
}
