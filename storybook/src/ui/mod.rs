use crate::ui::doc_page::DocPage;
use crate::ui::preview::StoryPage;
use crate::ui::sidebar::{ComponentInfo, ComponentTree, Selection};
use crate::{STORYBOOK_CSS, get_components, take_config};
use dioxus::prelude::*;
use lucide_dioxus::{Grid3X3, Maximize2, Minimize2, Moon, RotateCcw, Square, Sun, ZoomIn, ZoomOut};
use std::collections::HashMap;

pub mod doc_page;
pub mod preview;
pub mod props_editor;
pub mod sidebar;

/// Represents the available viewport size presets for story preview.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ViewportSize {
    FullWidth,
    SmallMobile,
    LargeMobile,
    Tablet,
}

impl ViewportSize {
    /// Returns the pixel width constraint, or `None` for full width.
    pub fn to_width(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "100%",
            ViewportSize::SmallMobile => "375px",
            ViewportSize::LargeMobile => "428px",
            ViewportSize::Tablet => "768px",
        }
    }

    /// Returns a human-readable label for display in the dropdown.
    pub fn label(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "Full Width",
            ViewportSize::SmallMobile => "Small Mobile (375px)",
            ViewportSize::LargeMobile => "Large Mobile (428px)",
            ViewportSize::Tablet => "Tablet (768px)",
        }
    }

    /// Returns a short string value used as the `<option>` value attribute.
    pub fn value(self) -> &'static str {
        match self {
            ViewportSize::FullWidth => "full",
            ViewportSize::SmallMobile => "375",
            ViewportSize::LargeMobile => "428",
            ViewportSize::Tablet => "768",
        }
    }

    /// Parse from the `<option>` value string.
    pub fn from_value(s: &str) -> Self {
        match s {
            "375" => ViewportSize::SmallMobile,
            "428" => ViewportSize::LargeMobile,
            "768" => ViewportSize::Tablet,
            _ => ViewportSize::FullWidth,
        }
    }

    /// All variants in display order.
    pub fn all() -> &'static [ViewportSize] {
        &[
            ViewportSize::FullWidth,
            ViewportSize::SmallMobile,
            ViewportSize::LargeMobile,
            ViewportSize::Tablet,
        ]
    }
}

/// Global UI settings shared via context
#[derive(Clone, Copy)]
pub(crate) struct UiSettings {
    pub is_dark_theme: Signal<bool>,
    pub grid_enabled: Signal<bool>,
    pub outline_enabled: Signal<bool>,
    pub fullscreen: Signal<bool>,
    pub zoom_level: Signal<i32>,
    pub viewport_width: Signal<ViewportSize>,
}

#[component]
pub(crate) fn App() -> Element {
    // Take the config from thread-local storage and provide it as context
    let _config = use_context_provider(take_config);

    // Provide UI settings as context
    let _ui_settings = use_context_provider(|| UiSettings {
        is_dark_theme: Signal::new(false),
        grid_enabled: Signal::new(false),
        outline_enabled: Signal::new(false),
        fullscreen: Signal::new(false),
        zoom_level: Signal::new(100),
        viewport_width: Signal::new(ViewportSize::FullWidth),
    });

    rsx! {
        Stylesheet { href: STORYBOOK_CSS }
        Storybook {}
    }
}

/// Top navigation bar with theme, grid, outline, fullscreen toggles, and story-specific controls
#[component]
fn TopBar(selected: Signal<Option<Selection>>) -> Element {
    let mut ui_settings = use_context::<UiSettings>();

    let is_dark = (ui_settings.is_dark_theme)();
    let grid_on = (ui_settings.grid_enabled)();
    let outline_on = (ui_settings.outline_enabled)();
    let fullscreen_on = (ui_settings.fullscreen)();
    let zoom = (ui_settings.zoom_level)();
    let viewport = (ui_settings.viewport_width)();

    let is_story_selected = matches!(selected(), Some(Selection::Story(_, _)));

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

                // Story-specific controls: zoom and viewport
                if is_story_selected {
                    div { class: "top-bar-divider" }

                    // Zoom controls
                    button {
                        class: "top-bar-btn",
                        title: "Zoom Out",
                        onclick: move |_| {
                            let current = (ui_settings.zoom_level)();
                            if current > 25 {
                                ui_settings.zoom_level.set(current - 25);
                            }
                        },
                        ZoomOut {}
                    }
                    span { class: "top-bar-zoom-level", "{zoom}%" }
                    button {
                        class: "top-bar-btn",
                        title: "Zoom In",
                        onclick: move |_| {
                            let current = (ui_settings.zoom_level)();
                            if current < 200 {
                                ui_settings.zoom_level.set(current + 25);
                            }
                        },
                        ZoomIn {}
                    }
                    button {
                        class: "top-bar-btn",
                        title: "Reset Zoom",
                        onclick: move |_| ui_settings.zoom_level.set(100),
                        RotateCcw {}
                    }

                    div { class: "top-bar-divider" }

                    // Viewport size dropdown
                    select {
                        class: "top-bar-viewport-select",
                        title: "Viewport Size",
                        value: "{viewport.value()}",
                        onchange: move |e: Event<FormData>| {
                            let size = ViewportSize::from_value(&e.value());
                            ui_settings.viewport_width.set(size);
                        },
                        for size in ViewportSize::all() {
                            option {
                                value: "{size.value()}",
                                selected: viewport == *size,
                                "{size.label()}"
                            }
                        }
                    }
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
    let mut search_query = use_signal(String::new);
    let selected = use_signal(|| Option::<Selection>::None);
    let components = use_store(|| ComponentStore {
        components: get_components()
            .map(|c| {
                (
                    c.name.to_string(),
                    ComponentInfo {
                        name: c.name.to_string(),
                        category: c.tag.to_string(),
                    },
                )
            })
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
            TopBar { selected }

            div { class: "content-wrapper",
                if !(ui_settings.fullscreen)() {
                    div { class: "sidebar",
                        div { class: "search-container",
                            input {
                                class: "search-input",
                                r#type: "text",
                                placeholder: "Search components...",
                                value: "{search_query}",
                                oninput: move |e| search_query.set(e.value()),
                            }
                        }
                        div { class: "component-tree",
                            ComponentTree {
                                components: filtered_components(),
                                selected,
                            }
                        }
                    }
                }

                div { class: "main-content",
                    div { class: "component-preview",
                        match selected() {
                            Some(Selection::Story(component_name, story_index)) => rsx! {
                                StoryPage {
                                    key: "{component_name}-{story_index}",
                                    component_name,
                                    story_index,
                                }
                            },
                            Some(Selection::DocPage(doc_path)) => rsx! {
                                DocPage { key: "{doc_path}", path: doc_path }
                            },
                            None => rsx! {
                                div { class: "empty-state",
                                    h2 { "Select a story" }
                                    p { "Choose a component and story from the sidebar to preview it" }
                                }
                            },
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
