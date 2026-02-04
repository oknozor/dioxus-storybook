use dioxus::prelude::*;
use schemars::schema::{InstanceType, RootSchema, Schema, SchemaObject, SingleOrVec};
use std::collections::HashMap;
use storybook::{find_component, StoryInfo};

mod components;
use components::{ComponentInfo, ComponentTree};

// Import cadence_ui to ensure the storybook registrations are linked
// and to get the UI CSS asset for iframe injection
use cadence_ui::UI_CSS;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Stylesheet { href: asset!("/assets/storybook.css") }
        Storybook {}
    }
}

#[derive(Store, PartialEq, Clone, Debug)]
struct ComponentStore {
    components: HashMap<String, ComponentInfo>,
}

#[store]
impl<Lens> Store<ComponentStore, Lens> {
    fn search(&self, query: &str) -> Vec<ComponentInfo> {
        self.components()
            .values()
            .filter(|c| {
                c.read().name.to_lowercase().contains(query)
                    || c.read().category.to_lowercase().contains(query)
            })
            .map(|c| c())
            .collect()
    }
}

#[component]
fn Storybook() -> Element {
    let mut search_query = use_signal(|| String::new());
    let selected_component = use_signal(|| Option::<String>::None);
    let components = use_store(|| ComponentStore {
        components: storybook::get_components().into_iter().map(|c| (c.name.to_string(), ComponentInfo {
            name: c.name.to_string(),
            category: c.tag.to_string(),
        }))
            .collect(),
    });

    let filtered_components = use_memo(move || components.search(&search_query()));

    rsx! {
        div { class: "storybook-container",
            div { class: "sidebar",
                div { class: "search-container",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search components...",
                        value: "{search_query}",
                        oninput: move |e| search_query.set(e.value())
                    }
                }
                div { class: "component-tree",
                    ComponentTree { components: filtered_components(), selected_component }
                }
            }

            // Main content area
            div { class: "main-content",
                div { class: "component-preview",
                    if let Some(component_name) = selected_component() {
                        ComponentPreview {
                            key: "{component_name}",
                            component_name
                        }
                    } else {
                        div { class: "empty-state",
                            h2 { "Select a component" }
                            p { "Choose a component from the sidebar to preview it" }
                        }
                    }
                }
            }
        }
    }
}

/// A single story card that renders one story with its own HTML capture and iframe
#[component]
fn StoryCard(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: fn(&str) -> Element,
    prop_schema: RootSchema,
    #[props(default)]
    attribute: Vec<Attribute>,
) -> Element {
    // Each story card has its own iframe HTML signal
    let mut iframe_html = use_signal(|| String::new());

    // Props JSON as a signal so it can be edited
    let props_json = use_signal(|| story.props_json.clone());

    // Props editor collapsed by default
    let mut props_expanded = use_signal(|| false);

    // Unique ID for this story's hidden render container
    let container_id = format!(
        "preview-render-{}-story-{}",
        component_name.replace(" ", "-").replace("::", "-"),
        story_index
    );
    let container_id_for_effect = container_id.clone();

    // Effect to capture rendered HTML and update iframe content
    // This effect re-runs whenever props_json changes
    use_effect(move || {
        // Read props_json to make this effect reactive to changes
        let _props_json_value = props_json();

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::window;
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(container) = document.get_element_by_id(&container_id_for_effect) {
                        let html = container.inner_html();
                        iframe_html.set(html);
                    }
                }
            }
        }
    });

    // Build the srcdoc content with CSS isolation
    let css_href = UI_CSS.to_string();
    let srcdoc = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <link rel="stylesheet" href="{}">
    <style>
        body {{ margin: 0; padding: 16px; }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        css_href,
        iframe_html()
    );

    rsx! {
        div { class: "story-card",
            // Story title
            h4 { class: "story-card-title", "{story.title}" }

            // Story description (if present)
            if let Some(desc) = &story.description {
                p { class: "story-card-description", "{desc}" }
            }

            // Hidden container where we render the component to capture its HTML
            div {
                id: "{container_id}",
                style: "position: absolute; visibility: hidden; pointer-events: none;",
                {(render_fn)(&props_json())}
            }

            // Iframe that displays the component with CSS isolation
            div { class: "story-preview-area",
                iframe { class: "preview-iframe", srcdoc: "{srcdoc}" }
            }

            // Collapsible props editor section
            div { class: "props-editor-section",
                div {
                    class: "props-editor-header",
                    onclick: move |_| props_expanded.toggle(),
                    span { class: "collapse-icon",
                        if props_expanded() {
                            "▼"
                        } else {
                            "▶"
                        }
                    }
                    "Props Editor"
                }
                if props_expanded() {
                    PropsEditor { props_json, schema: prop_schema.clone() }
                }
            }
        }
    }
}

#[component]
fn ComponentPreview(
    component_name: String,
    #[props(default)]
    attribute: Vec<Attribute>,
) -> Element {
    // Find the component registration
    let Some(registration) = find_component(&component_name) else {
        return rsx! {
            div { class: "error", "Component not found: {component_name}" }
        };
    };

    // Load stories directly - no need for use_effect since the component
    // is remounted when component_name changes (via key attribute on parent)
    let current_stories = (registration.get_stories)();
    let render_fn = registration.render_with_props;

    // Get the schema for this component's props
    let prop_schema = (registration.get_prop_schema)();

    rsx! {
        div { class: "preview-container",
            h2 { "{component_name}" }

            // Display all stories in a vertical layout
            div { class: "stories-container",
                for (index , story) in current_stories.iter().enumerate() {
                    StoryCard {
                        key: "{component_name}-{index}",
                        story: story.clone(),
                        component_name: component_name.clone(),
                        story_index: index,
                        render_fn,
                        prop_schema: prop_schema.clone()
                    }
                }
            }
        }
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

#[component]
fn PropsEditor(props_json: Signal<String>, schema: RootSchema) -> Element {
    let fields = extract_fields_from_schema(&schema);

    rsx! {
        div { class: "props-editor",
            if fields.is_empty() {
                div { class: "props-empty",
                    "No editable props available."
                    br {}
                    "Use #[storybook] on the Props struct for full editing support."
                }
            } else {
                for field in fields.iter() {
                    PropFieldEditor {
                        key: "{field.name}",
                        field: field.clone(),
                        props_json
                    }
                }
            }
        }
    }
}

#[component]
fn PropFieldEditor(field: SchemaFieldInfo, mut props_json: Signal<String>) -> Element {
    let field_name = field.name.clone();

    // Check if this is a non-editable field (unit type represented as null)
    let is_non_editable = field.instance_type == Some(InstanceType::Null);

    if is_non_editable {
        return rsx! {
            div { class: "prop-field non-editable",
                label { class: "prop-label", "{field_name}" }
                span { class: "prop-value-placeholder", "—" }
                span { class: "prop-type", "non-editable" }
            }
        };
    }

    // Get the current value for this field by parsing the JSON
    let current_value = serde_json::from_str::<serde_json::Value>(&props_json())
        .ok()
        .and_then(|v| v.get(&field_name).cloned())
        .map(|v| {
            if v.is_string() {
                v.as_str().unwrap_or("").to_string()
            } else {
                v.to_string()
            }
        })
        .unwrap_or_default();

    let field_name_for_handler = field_name.clone();
    let instance_type = field.instance_type;
    let type_name = field.type_name.clone();
    let required_marker = if field.is_required { "*" } else { "" };

    // Render different input types based on schema type
    match field.instance_type {
        Some(InstanceType::Boolean) => {
            let is_checked = current_value == "true";
            rsx! {
                div { class: "prop-field editable",
                    label { class: "prop-label", "{field_name}{required_marker}" }
                    input {
                        class: "prop-input prop-checkbox",
                        r#type: "checkbox",
                        checked: is_checked,
                        onchange: move |e| {
                            let new_value = e.checked();
                            update_prop_value(&mut props_json, &field_name_for_handler, serde_json::Value::Bool(new_value));
                        }
                    }
                    span { class: "prop-type", "{type_name}" }
                }
            }
        }
        Some(InstanceType::Integer) | Some(InstanceType::Number) => {
            rsx! {
                div { class: "prop-field editable",
                    label { class: "prop-label", "{field_name}{required_marker}" }
                    input {
                        class: "prop-input",
                        r#type: "number",
                        value: "{current_value}",
                        oninput: move |e| {
                            let new_value = e.value();
                            let parsed = parse_input_value(&new_value, instance_type);
                            update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                        }
                    }
                    span { class: "prop-type", "{type_name}" }
                }
            }
        }
        _ => {
            // Default to text input for strings and other types
            rsx! {
                div { class: "prop-field editable",
                    label { class: "prop-label", "{field_name}{required_marker}" }
                    input {
                        class: "prop-input",
                        r#type: "text",
                        value: "{current_value}",
                        oninput: move |e| {
                            let new_value = e.value();
                            let parsed = parse_input_value(&new_value, instance_type);
                            update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                        }
                    }
                    span { class: "prop-type", "{type_name}" }
                }
            }
        }
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
