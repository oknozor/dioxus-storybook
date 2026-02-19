use crate::ui::models::Selection;
use dioxus::prelude::*;
use lucide_dioxus::{BookOpen, ChevronRight, Component, FileText};

#[cfg(feature = "self-stories")]
use crate::{self as storybook};

#[cfg(feature = "self-stories")]
use storybook_macro::storybook;

/// Expandable sidebar node representing a single registered component.
///
/// When collapsed, shows the component name with a `Component` icon.
/// When expanded (`is_active = true`), reveals a list of story links and
/// an optional "Documentation" link (if the component has `///` doc
/// comments). This is a pure presentational component — `is_active` is
/// computed by the parent.
///
/// # Props
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `name` | `String` | — | The registered component name. |
/// | `selected` | `Signal<Option<Selection>>` | — | Currently selected sidebar item. |
/// | `stories` | `Vec<String>` | — | Titles of the component's stories. |
/// | `is_active` | `bool` | — | Whether this node is currently expanded. |
/// | `has_docs` | `bool` | `false` | Whether a "Documentation" link should be shown. |
///
/// @[story:Molecules/ComponentNode/Collapsed]
///
/// @[story:Molecules/ComponentNode/Expanded]
///
/// @[story:Molecules/ComponentNode/Expanded with Docs]
#[cfg_attr(feature = "self-stories", storybook(tag = "Molecules"))]
#[component]
pub fn ComponentNode(
    name: String,
    selected: Signal<Option<Selection>>,
    stories: Vec<String>,
    is_active: bool,
    #[props(default = false)] has_docs: bool,
) -> Element {
    let component_name = name.clone();
    let doc_path = format!("__component__/{}", name);

    rsx! {
        div { class: "component-node-group",
            RootNode { name: name.clone(), expanded: is_active, selected }
            if is_active {
                div { class: "story-children",
                    if has_docs {
                        {
                            let doc_path_click = doc_path.clone();
                            let is_doc_selected = selected() == Some(Selection::DocPage(doc_path.clone()));
                            rsx! {
                                div {
                                    class: if is_doc_selected { "doc-node selected" } else { "doc-node" },
                                    onclick: move |_| {
                                        selected.set(Some(Selection::DocPage(doc_path_click.clone())));
                                    },
                                    span { class: "doc-icon",
                                        FileText { size: 14, stroke_width: 2 }
                                    }
                                    span { class: "doc-name", "Documentation" }
                                }
                            }
                        }
                    }
                    for (index , story_title) in stories.iter().enumerate() {
                        {
                            let component_name = component_name.clone();
                            let is_selected = selected()
                                == Some(Selection::Story(component_name.clone(), index));
                            let story_title = story_title.clone();
                            rsx! {
                                StoryNode {
                                    key: "{component_name}-story-{index}",
                                    is_selected,
                                    onclick: move |_| selected.set(Some(Selection::Story(component_name.clone(), index))),
                                    story_title,
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
fn RootNode(name: String, expanded: bool, selected: Signal<Option<Selection>>) -> Element {
    rsx! {
        div {
            class: if expanded { "component-node active" } else { "component-node" },
            onclick: move |_| {
                selected.set(Some(Selection::Story(name.clone(), 0)));
            },
            span { class: if expanded { "arrow expanded" } else { "arrow" },
                ChevronRight { size: 12, stroke_width: 2 }
            }
            span { class: "component-icon",
                Component { size: 14, stroke_width: 2 }
            }
            span { class: "component-name", "{name}" }
        }
    }
}
#[component]
fn StoryNode(
    is_selected: bool,
    story_title: String,
    #[props(extends = GlobalAttributes, extends = tr)] attributes: Vec<Attribute>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: if is_selected { "story-node selected" } else { "story-node" },
            onclick: move |e| {
                onclick.call(e);
            },
            ..attributes,
            span { class: "story-icon",
                BookOpen { size: 12, stroke_width: 2 }
            }
            span { class: "story-name", "{story_title}" }
        }
    }
}
