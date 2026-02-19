use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    #[props(extends = GlobalAttributes, extends = input)]
    attributes: Vec<Attribute>,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        input {
            class: "prop-input prop-checkbox",
            r#type: "checkbox",
            onchange: move |e| onchange.call(e.checked()),
            ..attributes,
        }
    }
}

#[component]
pub fn TextInput(
    oninput: EventHandler<String>,
    #[props(extends = GlobalAttributes, extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        input {
            class: "prop-input",
            oninput: move |e| oninput.call(e.value()),
            ..attributes,
        }
    }
}

