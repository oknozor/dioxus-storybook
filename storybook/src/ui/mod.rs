use std::collections::HashMap;
use dioxus::prelude::*;
use crate::{get_components, take_config, StorybookConfig, STORYBOOK_CSS};
use crate::ui::preview::ComponentPreview;
use crate::ui::sidebar::{ComponentInfo, ComponentTree, Selection};
use crate::ui::doc_page::DocPage;

pub mod preview;
pub mod sidebar;
pub mod props_editor;
pub mod doc_page;
#[component]
pub (crate) fn App() -> Element {
    // Take the config from thread-local storage and provide it as context
    let _config = use_context_provider(|| take_config());

    rsx! {
        Stylesheet { href: STORYBOOK_CSS }
        Storybook {}
    }
}

#[component]
fn Storybook() -> Element {
    let mut search_query = use_signal(|| String::new());
    let selected = use_signal(|| Option::<Selection>::None);
    let components = use_store(|| ComponentStore {
        components: get_components().into_iter().map(|c| (c.name.to_string(), ComponentInfo {
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
                    ComponentTree { components: filtered_components(), selected }
                }
            }

            // Main content area
            div { class: "main-content",
                div { class: "component-preview",
                    match selected() {
                        Some(Selection::Component(component_name)) => rsx! {
                            ComponentPreview {
                                key: "{component_name}",
                                component_name
                            }
                        },
                        Some(Selection::DocPage(doc_path)) => rsx! {
                            DocPage {
                                key: "{doc_path}",
                                path: doc_path
                            }
                        },
                        None => rsx! {
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
}

#[derive(Store, PartialEq, Clone, Debug)]
pub(crate) struct ComponentStore {
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
