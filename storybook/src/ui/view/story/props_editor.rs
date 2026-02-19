use crate::ui::view::shared::{Checkbox, Td, TextInput, Tr};
use crate::{SchemaFieldInfo, extract_fields_from_schema, parse_input_value, update_prop_value};
use dioxus::prelude::*;
use lucide_dioxus::{ChevronDown, ChevronRight};
use schemars::Schema;

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Collapsible header for the props editor panel.
///
/// Displays a "Props Editor" label with a chevron icon that toggles
/// between expanded (▼) and collapsed (▶) states. Clicking the header
/// toggles the `expanded` signal, which controls whether the props
/// editing table below is visible.
///
/// # Props
///
/// | Prop | Type | Description |
/// |------|------|-------------|
/// | `expanded` | `Signal<bool>` | `true` = panel is open and the chevron points down. |
///
/// @[story:Molecules/PropsEditorHeader/Expanded]
///
/// @[story:Molecules/PropsEditorHeader/Collapsed]
#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn PropsEditorHeader(expanded: Signal<bool>) -> Element {
    rsx! {
        div { class: "props-editor-header", onclick: move |_| expanded.toggle(),
            span { class: "collapse-icon",
                if expanded() {
                    ChevronDown { size: 14, stroke_width: 2 }
                } else {
                    ChevronRight { size: 14, stroke_width: 2 }
                }
            }
            "Props Editor"
        }
    }
}

#[component]
pub(crate) fn PropsEditor(props_json: Signal<String>, schema: Schema) -> Element {
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
                table { class: "props-table",
                    thead {
                        tr {
                            th { "Name" }
                            th { "Type" }
                            th { "Description" }
                            th { "Value" }
                        }
                    }
                    tbody {
                        for field in fields.iter() {
                            PropFieldRow {
                                key: "{field.name}",
                                field: field.clone(),
                                props_json,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PropFieldRow(field: SchemaFieldInfo, mut props_json: Signal<String>) -> Element {
    let field_name = field.name.clone();
    let description = field.description.clone();
    let type_name = field.type_name.clone();

    // Check if this is a non-editable field (unit type represented as null)
    let is_non_editable = field.schema_type.as_deref() == Some("null");

    if is_non_editable {
        return rsx! {
            tr { class: "prop-row non-editable",
                td { class: "prop-cell prop-name", "{field_name}" }
                td { class: "prop-cell prop-type", "non-editable" }
                td { class: "prop-cell prop-description",
                    if let Some(desc) = &description {
                        "{desc}"
                    } else {
                        "—"
                    }
                }
                td { class: "prop-cell prop-value", "—" }
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
    let schema_type = field.schema_type.clone();
    let required_marker = if field.is_required { "*" } else { "" };

    let value_cell = match field.schema_type.as_deref() {
        Some("boolean") => {
            let is_checked = current_value == "true";
            rsx! {
                Checkbox {
                    checked: is_checked,
                    onchange: move |checked| {
                        update_prop_value(
                            &mut props_json,
                            &field_name_for_handler,
                            serde_json::Value::Bool(checked),
                        );
                    },
                }
            }
        }
        Some("integer") | Some("number") => {
            rsx! {
                TextInput {
                    r#type: "number",
                    value: "{current_value}",
                    oninput: move |e: String| {
                        let parsed = parse_input_value(&e, schema_type.as_deref());
                        update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                    },
                }
            }
        }
        _ => {
            let schema_type = schema_type.clone();
            rsx! {
                TextInput {
                    r#type: "text",
                    value: "{current_value}",
                    oninput: move |e: String| {
                        let parsed = parse_input_value(&e, schema_type.as_deref());
                        update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                    },
                }
            }
        }
    };

    rsx! {
        Tr {
            Td { class: "prop-name", "{field_name}{required_marker}" }
            Td { class: "prop-type", "{type_name}" }
            Td { class: "prop-description",
                if let Some(desc) = &description {
                    "{desc}"
                } else {
                    "—"
                }
            }
            Td { class: "prop-cell prop-value", {value_cell} }
        }
    }
}
