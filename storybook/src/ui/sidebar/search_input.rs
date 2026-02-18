use dioxus::prelude::*;

#[component]
pub fn SearchInput(search_query: Signal<String>) -> Element {
    rsx! {
        div { class: "search-container",
            input {
                class: "search-input",
                r#type: "text",
                placeholder: "Search components...",
                value: "{search_query}",
                oninput: move |e| search_query.set(e.value()),
            }
        }
    }
}
