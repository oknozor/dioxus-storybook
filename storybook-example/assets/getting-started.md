# Getting Started with Dioxus Storybook

Welcome to **Dioxus Storybook**, a component documentation and testing framework for Dioxus applications.

## Overview

Dioxus Storybook allows you to develop UI components in isolation, document them with markdown, and showcase different states through stories.

## Example Components

This storybook ships with four example components across three categories:

### Buttons

@[story:Examples/Buttons/ExampleButton/Default]

@[story:Examples/Buttons/ExampleButton/Disabled]

### Data Display

@[story:Examples/Data Display/ExampleCard/Default]

@[story:Examples/Data Display/ExampleCard/Long Content]

### Feedback

@[story:Examples/Feedback/ExampleBadge/Default]

@[story:Examples/Feedback/ExampleBadge/Success]

@[story:Examples/Feedback/ExampleAlert/Info]

@[story:Examples/Feedback/ExampleAlert/Error]

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

## Component Documentation

Add `///` doc comments with `@[story:...]` embeds directly on your component
functions. The storybook macro converts them to HTML and displays them as a
**Documentation** page in the sidebar:

```rust
/// A reusable button.
///
/// @[story:MyCategory/MyButton/Default]
#[storybook(tag = "MyCategory")]
#[component]
pub fn MyButton(label: String) -> Element {
    rsx! { button { "{label}" } }
}
```

## Markdown Documentation Pages

You can also add standalone documentation pages using the `storydoc!` macro:

```rust
storybook::storydoc!("Path/To/Page", "assets/my-docs.md");
```

Embed stories in your markdown using the `@[story:...]` syntax:

```markdown
@[story:Category/Component/Story]
```
