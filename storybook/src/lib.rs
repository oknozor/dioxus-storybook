//! # Dioxus Storybook
//!
//! A component development and documentation framework for [Dioxus](https://dioxuslabs.com/).
//! Develop, document, and visually test your UI components in isolation — inspired
//! by [Storybook.js](https://storybook.js.org/).
//!
//! ## Quick start
//!
//! ### 1. Annotate your component
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use storybook::{storybook, Stories, Story};
//!
//! #[storybook(tag = "Examples")]
//! #[component]
//! pub fn MyButton(label: String, #[props(default = false)] disabled: bool) -> Element {
//!     rsx! { button { disabled, "{label}" } }
//! }
//! ```
//!
//! ### 2. Implement the [`Stories`] trait
//!
//! ```rust,ignore
//! impl Stories for MyButtonProps {
//!     fn stories() -> Vec<Story<Self>> {
//!         vec![
//!             Story::new("Default", Self {
//!                 label: "Click me".to_string(),
//!                 disabled: false,
//!             }),
//!             Story::with_description(
//!                 "Disabled",
//!                 "A disabled button that cannot be clicked",
//!                 Self { label: "Can't click".to_string(), disabled: true },
//!             ),
//!         ]
//!     }
//! }
//! ```
//!
//! ### 3. Launch the storybook
//!
//! ```rust,ignore
//! fn main() {
//!     storybook::launch(
//!         storybook::StorybookConfig::default()
//!             .with_title("My Component Library"),
//!     );
//! }
//! ```
//!
//! ## Features
//!
//! - **Story-centric navigation** — sidebar tree with Category → Component → Story,
//!   just like Storybook.js.
//! - **Live props editor** — auto-generated from [`schemars::JsonSchema`]; edit props
//!   in real time and see the component update.
//! - **Decorators** — wrap stories with extra markup via [`Decorator`] functions
//!   (padding, theme providers, etc.).
//! - **Documentation pages** — embed Markdown docs in the sidebar with the
//!   [`storydoc!`] macro, including live story previews.
//! - **Dark / light theme** — toggle between themes from the toolbar.
//! - **Zero-config registration** — the [`storybook`](macro@storybook) attribute macro
//!   and the [`inventory`] crate handle compile-time discovery automatically.
//!
//! ## Categories and folders
//!
//! The `tag` parameter on `#[storybook]` controls sidebar placement. Use `/` to
//! create nested folders:
//!
//! ```rust,ignore
//! #[storybook(tag = "Forms/Inputs")]
//! #[component]
//! pub fn TextInput(/* ... */) -> Element { /* ... */ }
//! ```
//!
//! This produces a sidebar tree like:
//!
//! ```text
//! Forms/
//!   Inputs/
//!     TextInput
//!       Default
//!       With Placeholder
//! ```
//!
//! ## Documentation pages
//!
//! Register a Markdown file alongside a category:
//!
//! ```rust,ignore
//! storybook::storydoc!("Examples", "assets/getting-started.md");
//! ```
//!
//! Inside the Markdown you can embed live story previews with the `@[story:...]` syntax:
//!
//! ```markdown
//! @[story:Examples/ExampleCard/Default]
//! ```
//!
//! ## Re-exports
//!
//! This crate re-exports several dependencies so that downstream crates do not
//! need to depend on them directly:
//!
//! - [`dioxus`] — the Dioxus framework
//! - [`serde`] / [`serde_json`] — serialization
//! - [`schemars`] — JSON Schema generation (used for the props editor)
//! - [`inventory`] — compile-time component collection

pub use dioxus;
pub use inventory;
pub use schemars;
pub use serde;
pub use serde_json;
pub use storybook_macro::storybook;
pub use storybook_macro::storydoc;

use crate::ui::App;
use dioxus::prelude::*;
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};

const STORYBOOK_CSS: Asset = asset!("../assets/storybook.css");

mod ui;

/// Configuration for the storybook application.
///
/// Use the builder methods [`with_css`](Self::with_css) and
/// [`with_title`](Self::with_title) to customise the storybook, then pass
/// the config to [`launch()`].
///
/// # Example
///
/// ```rust,ignore
/// storybook::launch(
///     StorybookConfig::default()
///         .with_css(MY_CSS)
///         .with_title("My Component Library"),
/// );
/// ```
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
/// This is the main entry point for the storybook. It stores the
/// [`StorybookConfig`], then starts the Dioxus application which
/// automatically discovers all components registered with
/// [`#[storybook]`](macro@storybook).
///
/// # Example
///
/// ```rust,ignore
/// fn main() {
///     storybook::launch(
///         storybook::StorybookConfig::default()
///             .with_css(MY_CSS)
///             .with_title("My Component Library"),
///     );
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
    fn stories() -> Vec<Story<Self>>
    where
        Self: Sized;
}

/// Function pointer that renders a component from a JSON-encoded props string.
///
/// Generated automatically by the [`#[storybook]`](macro@storybook) macro.
pub type RenderWithPropsFn = fn(&str) -> Element;

/// Function pointer that returns all [`StoryInfo`] entries for a component.
///
/// Generated automatically by the [`#[storybook]`](macro@storybook) macro.
pub type GetStoriesFn = fn() -> Vec<StoryInfo>;

/// Function pointer that returns the JSON Schema ([`RootSchema`])
/// for a component's props struct.
///
/// Generated automatically by the [`#[storybook]`](macro@storybook) macro.
pub type GetPropSchemaFn = fn() -> schemars::schema::RootSchema;

/// Runtime representation of a story with serialized (JSON) props.
///
/// This is the type-erased counterpart of [`Story<T>`] — it is produced by
/// the generated code so the UI can work with stories without knowing the
/// concrete props type.
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
            .field(
                "decorators",
                &format!("[{} decorators]", self.decorators.len()),
            )
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

/// Compile-time registration record for a storybook component.
///
/// One of these is created for every `#[storybook]`-annotated component and
/// collected at link time via the [`inventory`] crate. You should not need
/// to construct this manually — use the macro instead.
pub struct ComponentRegistration {
    /// Component name (e.g. `"MyButton"`).
    pub name: &'static str,
    /// Sidebar category / folder path (e.g. `"Forms/Inputs"`).
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

/// Returns an iterator over every [`ComponentRegistration`] collected at
/// compile time (i.e. every component annotated with `#[storybook]`).
pub fn get_components() -> impl Iterator<Item = &'static ComponentRegistration> {
    inventory::iter::<ComponentRegistration>()
}

/// Look up a [`ComponentRegistration`] by its component name.
///
/// Returns `None` if no component with the given name has been registered.
pub fn find_component(name: &str) -> Option<&'static ComponentRegistration> {
    inventory::iter::<ComponentRegistration>().find(|c| c.name == name)
}

/// Compile-time registration record for a documentation page.
///
/// Created by the [`storydoc!`] macro. The Markdown source is converted to
/// HTML at compile time and stored in [`content_html`](Self::content_html).
/// The page appears as a "Documentation" link inside the matching sidebar
/// folder.
#[derive(Debug)]
pub struct DocRegistration {
    /// The path in the tree where this doc page belongs (e.g., "Buttons/Primary")
    pub path: &'static str,
    /// The HTML content of the documentation (converted from markdown)
    pub content_html: &'static str,
}

inventory::collect!(DocRegistration);

/// Returns an iterator over every [`DocRegistration`] collected at compile
/// time (i.e. every page registered with [`storydoc!`]).
pub fn get_docs() -> impl Iterator<Item = &'static DocRegistration> {
    inventory::iter::<DocRegistration>()
}

/// Look up a [`DocRegistration`] by its tree path.
///
/// Returns `None` if no documentation page with the given path has been
/// registered.
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
                    let instance_type = schema_obj.instance_type.as_ref().and_then(|t| match t {
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
    fields.sort_by(|a, b| match (a.is_required, b.is_required) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    fields
}

/// Get a human-readable type name from a schema object
fn get_type_name_from_schema(
    schema: &SchemaObject,
    _definitions: &schemars::Map<String, Schema>,
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
    if let Some(enum_values) = &schema.enum_values
        && !enum_values.is_empty()
    {
        return "enum".to_string();
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
    if let Ok(mut json_value) = serde_json::from_str::<serde_json::Value>(&props_json())
        && let Some(obj) = json_value.as_object_mut()
    {
        obj.insert(field_name.to_string(), value);
        if let Ok(new_json) = serde_json::to_string_pretty(&json_value) {
            props_json.set(new_json);
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
            .and_then(serde_json::Number::from_f64)
            .map(serde_json::Value::Number)
            .unwrap_or_else(|| serde_json::Value::String(value.to_string())),
        _ => {
            // Try to parse as JSON first (for objects, arrays, etc.)
            serde_json::from_str(value)
                .unwrap_or_else(|_| serde_json::Value::String(value.to_string()))
        }
    }
}
