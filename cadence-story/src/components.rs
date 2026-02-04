use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub category: String,
}

#[component]
pub fn ComponentTree(
    components: Vec<ComponentInfo>,
    mut selected_component: Signal<Option<String>>,
) -> Element {
    let grouped = {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for component in components.iter() {
            map.entry(component.category.clone())
                .or_insert_with(Vec::new)
                .push(component.name.clone());
        }
        map
    };

    info!("ComponentTree: {:?}", grouped.len());

    rsx! {
        div { class: "tree",
            for (category , component_names) in grouped.iter() {
                CategoryNode {
                    category: category.clone(),
                    components: component_names.clone(),
                    selected_component
                }
            }
        }
    }
}

#[component]
fn CategoryNode(
    category: String,
    components: Vec<String>,
    mut selected_component: Signal<Option<String>>,
) -> Element {
    let mut expanded = use_signal(|| true);

    rsx! {
        div { class: "category-node",
            div {
                class: "category-header",
                onclick: move |_| expanded.set(!expanded()),
                span { class: if expanded() { "arrow expanded" } else { "arrow" }, "▶" }
                span { class: "category-name", "{category}" }
            }
            if expanded() {
                div { class: "category-children",
                    for component_name in &components {
                        {
                            let component_name = component_name.clone();
                            rsx! {
                                ComponentNode {
                                    name: component_name.clone(),
                                    selected: selected_component() == Some(component_name.clone()),
                                    onclick: move |_| {
                                        selected_component.set(Some(component_name.clone()));
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ComponentNode(name: String, selected: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        div { class: if selected { "component-node selected" } else { "component-node" }, onclick: move |_| onclick.call(()),
            span { class: "component-icon", "📦" }
            span { class: "component-name", "{name}" }
        }
    }
}
