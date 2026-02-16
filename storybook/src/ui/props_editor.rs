use crate::{SchemaFieldInfo, extract_fields_from_schema, parse_input_value, update_prop_value};
use dioxus::prelude::*;
use schemars::schema::{InstanceType, RootSchema};

#[component]
pub(crate) fn PropsEditor(props_json: Signal<String>, schema: RootSchema) -> Element {
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
    let is_non_editable = field.instance_type == Some(InstanceType::Null);

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
    let instance_type = field.instance_type;
    let required_marker = if field.is_required { "*" } else { "" };

    let value_cell = match field.instance_type {
        Some(InstanceType::Boolean) => {
            let is_checked = current_value == "true";
            rsx! {
                input {
                    class: "prop-input prop-checkbox",
                    r#type: "checkbox",
                    checked: is_checked,
                    onchange: move |e| {
                        let new_value = e.checked();
                        update_prop_value(
                            &mut props_json,
                            &field_name_for_handler,
                            serde_json::Value::Bool(new_value),
                        );
                    },
                }
            }
        }
        Some(InstanceType::Integer) | Some(InstanceType::Number) => {
            rsx! {
                input {
                    class: "prop-input",
                    r#type: "number",
                    value: "{current_value}",
                    oninput: move |e| {
                        let new_value = e.value();
                        let parsed = parse_input_value(&new_value, instance_type);
                        update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                    },
                }
            }
        }
        _ => {
            rsx! {
                input {
                    class: "prop-input",
                    r#type: "text",
                    value: "{current_value}",
                    oninput: move |e| {
                        let new_value = e.value();
                        let parsed = parse_input_value(&new_value, instance_type);
                        update_prop_value(&mut props_json, &field_name_for_handler, parsed);
                    },
                }
            }
        }
    };

    rsx! {
        tr { class: "prop-row editable",
            td { class: "prop-cell prop-name", "{field_name}{required_marker}" }
            td { class: "prop-cell prop-type", "{type_name}" }
            td { class: "prop-cell prop-description",
                if let Some(desc) = &description {
                    "{desc}"
                } else {
                    "—"
                }
            }
            td { class: "prop-cell prop-value", {value_cell} }
        }
    }
}
