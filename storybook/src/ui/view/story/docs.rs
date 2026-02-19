use dioxus::prelude::*;

#[component]
pub fn StoryDocs(docs: &'static str) -> Element {
    rsx! {
        div { class: "component-description", dangerous_inner_html: "{docs}" }
    }
}
