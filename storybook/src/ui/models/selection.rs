/// Information about a registered component.
#[cfg_attr(feature = "self-stories", derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema))]
#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

/// Selection type - a story, component, or doc page
#[cfg_attr(feature = "self-stories", derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema))]
#[derive(Clone, PartialEq, Debug)]
pub enum Selection {
    /// A specific story within a component (component_name, story_index)
    Story(String, usize),
    /// A documentation page
    DocPage(String),
}

/// The type of node in the hierarchy tree
#[cfg_attr(feature = "self-stories", derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    /// Top-level category (first segment of the path)
    Category,
    /// Intermediate folder (middle segments of the path)
    Folder,
}

