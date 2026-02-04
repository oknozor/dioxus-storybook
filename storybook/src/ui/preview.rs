use crate::ui::props_editor::PropsEditor;
use crate::{StoryInfo, StorybookConfig, find_component};
use dioxus::prelude::*;
use schemars::schema::RootSchema;

#[component]
pub(crate) fn ComponentPreview(
    component_name: String,
    #[props(default)] attribute: Vec<Attribute>,
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

/// A single story card that renders one story with its own HTML capture and iframe
#[component]
fn StoryCard(
    story: StoryInfo,
    component_name: String,
    story_index: usize,
    render_fn: fn(&str) -> Element,
    prop_schema: RootSchema,
    #[props(default)] attribute: Vec<Attribute>,
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

    // Get the config from context to access component CSS
    let config = use_context::<StorybookConfig>();

    // Build CSS link tags from config
    let css_links = config
        .component_css
        .iter()
        .map(|css| format!(r#"<link rel="stylesheet" href="{}">"#, css))
        .collect::<Vec<_>>()
        .join("\n    ");

    // Build the srcdoc content with CSS isolation
    let srcdoc = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    {css_links}
    <style>
        body {{ margin: 0; padding: 16px; }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
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
