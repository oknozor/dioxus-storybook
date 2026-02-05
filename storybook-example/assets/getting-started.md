# Getting Started with Dioxus Storybook

Welcome to **Dioxus Storybook**, a component documentation and testing framework for Dioxus applications.

## Overview

Dioxus Storybook allows you to develop UI components in isolation, document them with markdown, and showcase different states through stories.

## Example Components

Below are some example components registered in this storybook.

### Example Button

The `ExampleButton` component demonstrates a simple interactive button with customizable label and disabled state.

![Default](Examples/ExampleButton/Default)

Here's the disabled variant:

![Disabled](Examples/ExampleButton/Disabled)

### Example Card

The `ExampleCard` component shows how to create a card with a title and content.

![Default](Examples/ExampleCard/Default)

![With Long Content](Examples/ExampleCard/With Long Content)

## Usage

To add your own components to the storybook:

1. Add the `#[storybook(tag = "Category")]` attribute before your component
2. Implement the `Stories` trait on your Props struct
3. Define your stories with different prop configurations

```rust
#[storybook(tag = "MyCategory")]
#[component]
pub fn MyComponent(label: String) -> Element {
    rsx! { div { "{label}" } }
}

impl storybook::Stories for MyComponentProps {
    fn stories() -> Vec<storybook::Story<Self>> {
        vec![
            storybook::Story::new("Default", Self {
                label: "Hello".to_string(),
            }),
        ]
    }
}
```

## Documentation Pages

You can also add documentation pages using the `storydoc!` macro:

```rust
storybook::storydoc!("Path/To/Page", "assets/my-docs.md");
```

Embed stories in your markdown using image link syntax:

```markdown
![Story Name](Category/Component/Story)
```

