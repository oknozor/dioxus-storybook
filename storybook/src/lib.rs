//! Cadence Storybook - Component documentation and testing framework
//!
//! Use the `#[storybook(tag = "Category")]` attribute to register components.
//!
//! Components must implement the `Stories` trait to provide story configurations
//! for the storybook UI.

pub use dioxus;
pub use inventory;
pub use serde;
pub use serde_json;
pub use storybook_macro::storybook;

use dioxus::prelude::*;

/// A single story configuration for a component.
///
/// Each story represents a specific state or configuration of the component
/// that should be displayed in the storybook.
#[derive(Clone)]
pub struct Story<T> {
    /// The title of the story (e.g., "Default", "Loading State", "Error State")
    pub title: &'static str,
    /// Optional description explaining this story
    pub description: Option<&'static str>,
    /// The props to render the component with
    pub props: T,
}

impl<T> Story<T> {
    /// Create a new story with just a title and props
    pub fn new(title: &'static str, props: T) -> Self {
        Self {
            title,
            description: None,
            props,
        }
    }

    /// Create a new story with title, description, and props
    pub fn with_description(title: &'static str, description: &'static str, props: T) -> Self {
        Self {
            title,
            description: Some(description),
            props,
        }
    }
}

/// Trait for providing story configurations for a component.
///
/// Implement this trait on your component's Props struct to provide
/// meaningful demo configurations for the storybook.
///
/// # Example
/// ```ignore
/// #[cfg(feature = "storybook")]
/// impl storybook::Stories for MyComponentProps {
///     fn stories() -> Vec<storybook::Story<Self>> {
///         vec![
///             storybook::Story::new("Default", Self {
///                 name: "Demo Name".to_string(),
///                 count: 42,
///             }),
///             storybook::Story::with_description(
///                 "Empty State",
///                 "Shows the component with no data",
///                 Self {
///                     name: "".to_string(),
///                     count: 0,
///                 }
///             ),
///         ]
///     }
/// }
/// ```
pub trait Stories {
    fn stories() -> Vec<Story<Self>> where Self: Sized;
}

/// Type alias for the render function that takes JSON props
pub type RenderWithPropsFn = fn(&str) -> Element;

/// Type alias for getting all stories with their props as JSON
pub type GetStoriesFn = fn() -> Vec<StoryInfo>;

/// Type alias for getting prop field info
pub type GetPropFieldsFn = fn() -> Vec<PropFieldInfo>;

/// Runtime representation of a story with serialized props
#[derive(Clone, Debug, PartialEq)]
pub struct StoryInfo {
    /// The title of the story
    pub title: String,
    /// Optional description of the story
    pub description: Option<String>,
    /// The props serialized as JSON
    pub props_json: String,
}

/// Information about a single prop field for the UI
#[derive(Clone, Debug, PartialEq)]
pub struct PropFieldInfo {
    /// The name of the field
    pub name: &'static str,
    /// Whether this field is editable (false for EventHandler, Element, etc.)
    pub editable: bool,
    /// The type name for display purposes
    pub type_name: &'static str,
}

/// Registration info for a storybook component
pub struct ComponentRegistration {
    pub name: &'static str,
    pub tag: &'static str,
    /// Renders the component with props from JSON string
    pub render_with_props: RenderWithPropsFn,
    /// Gets all stories for this component
    pub get_stories: GetStoriesFn,
    /// Gets information about each prop field
    pub get_prop_fields: GetPropFieldsFn,
}

impl std::fmt::Debug for ComponentRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentRegistration")
            .field("name", &self.name)
            .field("tag", &self.tag)
            .finish()
    }
}

inventory::collect!(ComponentRegistration);

/// Get all registered components
pub fn get_components() -> impl Iterator<Item = &'static ComponentRegistration> {
    inventory::iter::<ComponentRegistration>()
}

/// Find a component by name
pub fn find_component(name: &str) -> Option<&'static ComponentRegistration> {
    inventory::iter::<ComponentRegistration>().find(|c| c.name == name)
}
