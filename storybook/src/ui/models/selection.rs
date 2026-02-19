use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Information about a registered component.
#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

/// Selection type - a story, component, or doc page
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Selection {
    /// A specific story within a component (component_name, story_index)
    Story(String, usize),
    /// A documentation page
    DocPage(String),
}

/// The type of node in the hierarchy tree
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    /// Top-level category (first segment of the path)
    Category,
    /// Intermediate folder (middle segments of the path)
    Folder,
}

