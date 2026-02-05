use std::collections::HashMap;
use dioxus::prelude::*;
use lucide_dioxus::{Sun, Moon, Grid3X3, Square, Maximize2, Minimize2};
use crate::{get_components, take_config, StorybookConfig, STORYBOOK_CSS};
use crate::ui::preview::ComponentPreview;
use crate::ui::sidebar::{ComponentInfo, ComponentTree, Selection};
use crate::ui::doc_page::DocPage;

pub mod preview;
pub mod sidebar;
pub mod props_editor;
pub mod doc_page;

/// Global UI settings shared via context
#[derive(Clone, Copy)]
pub(crate) struct UiSettings {
    pub is_dark_theme: Signal<bool>,
    pub grid_enabled: Signal<bool>,
    pub outline_enabled: Signal<bool>,
    pub fullscreen: Signal<bool>,
}

#[component]
pub(crate) fn App() -> Element {
    // Take the config from thread-local storage and provide it as context
    let _config = use_context_provider(|| take_config());

    // Provide UI settings as context
    let _ui_settings = use_context_provider(|| UiSettings {
        is_dark_theme: Signal::new(false),
        grid_enabled: Signal::new(false),
        outline_enabled: Signal::new(false),
        fullscreen: Signal::new(false),
    });

    rsx! {
        Stylesheet { href: STORYBOOK_CSS }
        Storybook {}
    }
}

/// Top navigation bar with theme, grid, outline, and fullscreen toggles
#[component]
fn TopBar() -> Element {
    let mut ui_settings = use_context::<UiSettings>();

    let is_dark = (ui_settings.is_dark_theme)();
    let grid_on = (ui_settings.grid_enabled)();
    let outline_on = (ui_settings.outline_enabled)();
    let fullscreen_on = (ui_settings.fullscreen)();

    rsx! {
        div { class: "top-bar",
            div { class: "top-bar-left",
                // Theme toggle
                button {
                    class: if is_dark { "top-bar-btn active" } else { "top-bar-btn" },
                    title: if is_dark { "Switch to light theme" } else { "Switch to dark theme" },
                    onclick: move |_| ui_settings.is_dark_theme.set(!is_dark),
                    if is_dark {
                        Sun {}
                    } else {
                        Moon {}
                    }
                }
                // Grid toggle
                button {
                    class: if grid_on { "top-bar-btn active" } else { "top-bar-btn" },
                    title: if grid_on { "Hide grid overlay" } else { "Show grid overlay" },
                    onclick: move |_| ui_settings.grid_enabled.set(!grid_on),
                    Grid3X3 {}
                }
                // Outline toggle
                button {
                    class: if outline_on { "top-bar-btn active" } else { "top-bar-btn" },
                    title: if outline_on { "Hide element outlines" } else { "Show element outlines" },
                    onclick: move |_| ui_settings.outline_enabled.set(!outline_on),
                    Square {}
                }
            }
            div { class: "top-bar-right",
                // Fullscreen toggle
                button {
                    class: if fullscreen_on { "top-bar-btn active" } else { "top-bar-btn" },
                    title: if fullscreen_on { "Show sidebar" } else { "Hide sidebar" },
                    onclick: move |_| ui_settings.fullscreen.set(!fullscreen_on),
                    if fullscreen_on {
                        Minimize2 {}
                    } else {
                        Maximize2 {}
                    }
                }
            }
        }
    }
}

#[component]
fn Storybook() -> Element {
    let ui_settings = use_context::<UiSettings>();
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

    let container_class = use_memo(move || {
        let mut classes = vec!["storybook-container"];
        if (ui_settings.is_dark_theme)() {
            classes.push("dark-theme");
        }
        if (ui_settings.fullscreen)() {
            classes.push("fullscreen-mode");
        }
        classes.join(" ")
    });

    rsx! {
        div { class: "{container_class}",
            TopBar {}

            div { class: "content-wrapper",
                if !(ui_settings.fullscreen)() {
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
