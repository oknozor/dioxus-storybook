use dioxus::prelude::*;
use std::collections::HashMap;
use storybook::{find_component, PropFieldInfo, StoryInfo};

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
    prop_fields: Vec<PropFieldInfo>,
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
                    PropsEditor { props_json, prop_fields: prop_fields.clone() }
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
                        prop_fields: (registration.get_prop_fields)()
                    }
                }
            }
        }
    }
}

#[component]
fn PropsEditor(mut props_json: Signal<String>, prop_fields: Vec<PropFieldInfo>) -> Element {
    debug!("PropFieldInfo: {:?}", prop_fields);
    debug!("Signal<String>: {:?}", props_json());
    rsx! {
        div { class: "props-editor",
            if prop_fields.is_empty() {
                div { class: "props-empty",
                    "No editable props available."
                    br {}
                    "Use #[storybook] on the Props struct for full editing support."
                }
            } else {
                for field in prop_fields.iter() {
                    PropFieldEditor { field: field.clone(), props_json }
                }
            }
        }
    }
}

#[component]
fn PropFieldEditor(field: PropFieldInfo, mut props_json: Signal<String>) -> Element {
    let field_name = field.name;

    if !field.editable {
        // Non-editable field (EventHandler, Callback, etc.)
        return rsx! {
            div { class: "prop-field non-editable",
                label { class: "prop-label", "{field_name}" }
                span { class: "prop-value-placeholder", "—" }
                span { class: "prop-type", "{field.type_name}" }
            }
        };
    }

    // Get the current value for this field by parsing the JSON
    let current_value = serde_json::from_str::<serde_json::Value>(&props_json())
        .ok()
        .and_then(|v| v.get(field_name).cloned())
        .map(|v| {
            if v.is_string() {
                v.as_str().unwrap_or("").to_string()
            } else {
                v.to_string()
            }
        })
        .unwrap_or_default();

    let field_name_owned = field_name.to_string();
    let field_type = field.type_name.to_string();

    rsx! {
        div { class: "prop-field editable",
            label { class: "prop-label", "{field_name}" }
            input {
                class: "prop-input",
                r#type: "text",
                value: "{current_value}",
                oninput: move |e| {
                    let new_value = e.value();
                    if let Ok(mut json_value) = serde_json::from_str::<
                        serde_json::Value,
                    >(&props_json()) {
                        if let Some(obj) = json_value.as_object_mut() {
                            let parsed_value = parse_input_value(&new_value, &field_type);
                            obj.insert(field_name_owned.clone(), parsed_value);
                            if let Ok(new_json) = serde_json::to_string_pretty(&json_value) {
                                props_json.set(new_json);
                            }
                        }
                    }
                }
            }
            span { class: "prop-type", "{field.type_name}" }
        }
    }
}

/// Parse an input string value into the appropriate JSON value based on type hint
fn parse_input_value(value: &str, type_hint: &str) -> serde_json::Value {
    // Try to parse based on type hint
    match type_hint {
        "bool" => value
            .parse::<bool>()
            .map(serde_json::Value::Bool)
            .unwrap_or_else(|_| serde_json::Value::String(value.to_string())),
        "i32" | "i64" | "u32" | "u64" | "usize" | "isize" => value
            .parse::<i64>()
            .map(|n| serde_json::Value::Number(n.into()))
            .unwrap_or_else(|_| serde_json::Value::String(value.to_string())),
        "f32" | "f64" => value
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
