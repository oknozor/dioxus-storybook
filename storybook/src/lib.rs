//! Dioxus Storybook - Component documentation and testing framework
//!
//! Use the `#[storybook(tag = "Category")]` attribute to register components.
//!
//! Components must implement the `Stories` trait to provide story configurations
//! for the storybook UI.
//!
//! # Example
//! ```ignore
//! use storybook::StorybookConfig;
//! use my_component_lib::MY_CSS;
//!
//! fn main() {
//!     storybook::launch(StorybookConfig {
//!         component_css: vec![MY_CSS.to_string()],
//!         title: Some("My Component Library".to_string()),
//!     });
//! }
//! ```

pub use dioxus;
pub use inventory;
pub use schemars;
pub use serde;
pub use serde_json;
pub use storybook_macro::storybook;

use dioxus::prelude::*;
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};
use crate::ui::App;

const STORYBOOK_CSS: Asset = asset!("../assets/storybook.css");

mod ui;

/// Configuration for the storybook application.
///
/// This struct allows users to customize the storybook with their own CSS
/// and other settings.
#[derive(Clone, Default)]
pub struct StorybookConfig {
    /// CSS URLs to inject into the component preview iframes.
    /// This should include the CSS for your component library.
    pub component_css: Vec<Asset>,
    /// Optional title for the storybook (displayed in the header).
    pub title: Option<String>,
}

impl StorybookConfig {
    /// Create a new StorybookConfig with the given CSS URLs.
    pub fn with_css(mut self, component_css: Asset) -> Self {
        self.component_css.push(component_css);
        self
    }

    /// Set the title for the storybook.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Launch the storybook application with the given configuration.
///
/// This is the main entry point for the storybook. It sets up the configuration
/// context and launches the Dioxus application.
///
/// # Example
/// ```ignore
/// use storybook::StorybookConfig;
/// use my_component_lib::MY_CSS;
///
/// fn main() {
///     storybook::launch(StorybookConfig {
///         component_css: vec![MY_CSS.to_string()],
///         title: Some("My Component Library".to_string()),
///     });
/// }
/// ```
pub fn launch(config: StorybookConfig) {
    // Store the config in a static so the App component can access it
    // We use a context provider inside App to make it available to child components
    CONFIG.with(|c| *c.borrow_mut() = Some(config));
    dioxus::launch(App);
}

// Thread-local storage for the config (set before launch, read by App)
std::thread_local! {
    static CONFIG: std::cell::RefCell<Option<StorybookConfig>> = const { std::cell::RefCell::new(None) };
}

/// Get the stored configuration (called by App during initialization)
pub(crate) fn take_config() -> StorybookConfig {
    CONFIG.with(|c| c.borrow_mut().take()).unwrap_or_default()
}

/// Type alias for a decorator function.
///
/// A decorator wraps a story's rendered element to add extra markup,
/// styling, or context. Decorators are applied in order, with the first
/// decorator being the outermost wrapper.
///
/// # Example
/// ```ignore
/// use storybook::Decorator;
/// use dioxus::prelude::*;
///
/// fn with_padding(story: Element) -> Element {
///     rsx! {
///         div { style: "padding: 20px;", {story} }
///     }
/// }
///
/// fn with_dark_background(story: Element) -> Element {
///     rsx! {
///         div { style: "background: #333; color: white;", {story} }
///     }
/// }
/// ```
pub type Decorator = fn(Element) -> Element;

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
    /// Optional decorators to wrap the story rendering.
    /// Decorators are applied in order, with the first decorator being the outermost wrapper.
    pub decorators: Vec<Decorator>,
}

impl<T> Story<T> {
    /// Create a new story with just a title and props
    pub fn new(title: &'static str, props: T) -> Self {
        Self {
            title,
            description: None,
            props,
            decorators: Vec::new(),
        }
    }

    /// Create a new story with title, description, and props
    pub fn with_description(title: &'static str, description: &'static str, props: T) -> Self {
        Self {
            title,
            description: Some(description),
            props,
            decorators: Vec::new(),
        }
    }

    /// Add a decorator to this story.
    ///
    /// Decorators wrap the story's rendered element. Multiple decorators
    /// are applied in order, with the first decorator being the outermost wrapper.
    ///
    /// # Example
    /// ```ignore
    /// Story::new("With Padding", MyProps::default())
    ///     .with_decorator(|story| rsx! {
    ///         div { style: "padding: 20px;", {story} }
    ///     })
    /// ```
    pub fn with_decorator(mut self, decorator: Decorator) -> Self {
        self.decorators.push(decorator);
        self
    }

    /// Add multiple decorators to this story.
    ///
    /// Decorators are applied in order, with the first decorator being the outermost wrapper.
    pub fn with_decorators(mut self, decorators: impl IntoIterator<Item = Decorator>) -> Self {
        self.decorators.extend(decorators);
        self
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

/// Type alias for getting the JSON schema for props
pub type GetPropSchemaFn = fn() -> schemars::schema::RootSchema;

/// Runtime representation of a story with serialized props
#[derive(Clone)]
pub struct StoryInfo {
    /// The title of the story
    pub title: String,
    /// Optional description of the story
    pub description: Option<String>,
    /// The props serialized as JSON
    pub props_json: String,
    /// Decorators to wrap the story rendering
    pub decorators: Vec<Decorator>,
}

impl std::fmt::Debug for StoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoryInfo")
            .field("title", &self.title)
            .field("description", &self.description)
            .field("props_json", &self.props_json)
            .field("decorators", &format!("[{} decorators]", self.decorators.len()))
            .finish()
    }
}

impl PartialEq for StoryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.props_json == other.props_json
            && self.decorators.len() == other.decorators.len()
            // Compare function pointers by address
            && self.decorators.iter().zip(other.decorators.iter())
                .all(|(a, b)| (*a as usize) == (*b as usize))
    }
}


/// Information about a property field extracted from JSON Schema
#[derive(Clone, Debug, PartialEq)]
struct SchemaFieldInfo {
    name: String,
    type_name: String,
    instance_type: Option<InstanceType>,
    is_required: bool,
    description: Option<String>,
}


/// Registration info for a storybook component
pub struct ComponentRegistration {
    pub name: &'static str,
    pub tag: &'static str,
    /// Component description extracted from doc comments (HTML format)
    pub description: &'static str,
    /// Renders the component with props from JSON string
    pub render_with_props: RenderWithPropsFn,
    /// Gets all stories for this component
    pub get_stories: GetStoriesFn,
    /// Gets the JSON schema for the props struct
    pub get_prop_schema: GetPropSchemaFn,
}

impl std::fmt::Debug for ComponentRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentRegistration")
            .field("name", &self.name)
            .field("tag", &self.tag)
            .field("description", &self.description)
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

/// Registration info for a documentation page.
///
/// Documentation pages are markdown files that can be associated with
/// a category, folder, or component path in the storybook tree.
/// They are displayed as the first item in a folder.
#[derive(Debug)]
pub struct DocRegistration {
    /// The path in the tree where this doc page belongs (e.g., "Buttons/Primary")
    pub path: &'static str,
    /// The HTML content of the documentation (converted from markdown)
    pub content_html: &'static str,
}

inventory::collect!(DocRegistration);

/// Get all registered documentation pages
pub fn get_docs() -> impl Iterator<Item = &'static DocRegistration> {
    inventory::iter::<DocRegistration>()
}

/// Find a documentation page by path
pub fn find_doc(path: &str) -> Option<&'static DocRegistration> {
    inventory::iter::<DocRegistration>().find(|d| d.path == path)
}

/// Extract field information from a JSON Schema
fn extract_fields_from_schema(schema: &RootSchema) -> Vec<SchemaFieldInfo> {
    let mut fields = Vec::new();

    // Get the required fields set
    let required: std::collections::HashSet<_> = schema
        .schema
        .object
        .as_ref()
        .map(|obj| obj.required.iter().cloned().collect())
        .unwrap_or_default();

    // Get properties from the schema
    if let Some(obj) = &schema.schema.object {
        for (name, prop_schema) in &obj.properties {
            let (type_name, instance_type, description) = match prop_schema {
                Schema::Object(schema_obj) => {
                    let instance_type = schema_obj
                        .instance_type
                        .as_ref()
                        .and_then(|t| match t {
                            SingleOrVec::Single(t) => Some(**t),
                            SingleOrVec::Vec(v) => v.first().copied(),
                        });
                    let type_name = get_type_name_from_schema(schema_obj, &schema.definitions);
                    let desc = schema_obj
                        .metadata
                        .as_ref()
                        .and_then(|m| m.description.clone());
                    (type_name, instance_type, desc)
                }
                Schema::Bool(_) => ("any".to_string(), None, None),
            };

            fields.push(SchemaFieldInfo {
                name: name.clone(),
                type_name,
                instance_type,
                is_required: required.contains(name),
                description,
            });
        }
    }

    // Sort fields: required first, then alphabetically
    fields.sort_by(|a, b| {
        match (a.is_required, b.is_required) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    fields
}

/// Get a human-readable type name from a schema object
fn get_type_name_from_schema(
    schema: &SchemaObject,
    definitions: &schemars::Map<String, Schema>,
) -> String {
    // Check for $ref first
    if let Some(ref_path) = &schema.reference {
        // Extract the type name from the reference path (e.g., "#/definitions/MyType" -> "MyType")
        return ref_path.rsplit('/').next().unwrap_or("unknown").to_string();
    }

    // Check instance type
    if let Some(instance_type) = &schema.instance_type {
        match instance_type {
            SingleOrVec::Single(t) => return format_instance_type(**t),
            SingleOrVec::Vec(types) => {
                let type_strs: Vec<_> = types.iter().map(|t| format_instance_type(*t)).collect();
                return type_strs.join(" | ");
            }
        }
    }

    // Check for enum values
    if let Some(enum_values) = &schema.enum_values {
        if !enum_values.is_empty() {
            return "enum".to_string();
        }
    }

    "unknown".to_string()
}

/// Format an instance type as a string
fn format_instance_type(t: InstanceType) -> String {
    match t {
        InstanceType::Null => "null".to_string(),
        InstanceType::Boolean => "bool".to_string(),
        InstanceType::Object => "object".to_string(),
        InstanceType::Array => "array".to_string(),
        InstanceType::Number => "number".to_string(),
        InstanceType::String => "String".to_string(),
        InstanceType::Integer => "integer".to_string(),
    }
}



/// Update a property value in the props JSON
fn update_prop_value(props_json: &mut Signal<String>, field_name: &str, value: serde_json::Value) {
    if let Ok(mut json_value) = serde_json::from_str::<serde_json::Value>(&props_json()) {
        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(field_name.to_string(), value);
            if let Ok(new_json) = serde_json::to_string_pretty(&json_value) {
                props_json.set(new_json);
            }
        }
    }
}

/// Parse an input string value into the appropriate JSON value based on schema type
fn parse_input_value(value: &str, instance_type: Option<InstanceType>) -> serde_json::Value {
    match instance_type {
        Some(InstanceType::Boolean) => value
            .parse::<bool>()
            .map(serde_json::Value::Bool)
            .unwrap_or_else(|_| serde_json::Value::String(value.to_string())),
        Some(InstanceType::Integer) => value
            .parse::<i64>()
            .map(|n| serde_json::Value::Number(n.into()))
            .unwrap_or_else(|_| serde_json::Value::String(value.to_string())),
        Some(InstanceType::Number) => value
            .parse::<f64>()
            .ok()
            .and_then(|n| serde_json::Number::from_f64(n))
            .map(serde_json::Value::Number)
            .unwrap_or_else(|| serde_json::Value::String(value.to_string())),
        _ => {
            // Try to parse as JSON first (for objects, arrays, etc.)
            serde_json::from_str(value)
                .unwrap_or_else(|_| serde_json::Value::String(value.to_string()))
        }
    }
}
