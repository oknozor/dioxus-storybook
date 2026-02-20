# Dioxus Storybook

A component development and documentation framework for [Dioxus](https://dioxuslabs.com/).
Develop, document, and visually test your UI components in isolation — inspired
by [Storybook.js](https://storybook.js.org/).

> **⚠️ Early Development — Not Stable**
>
> This project is in early development. APIs, features, and the overall architecture
> are subject to breaking changes without notice. Use at your own risk and pin to
> an exact version if you depend on it. Feedback and contributions are welcome!

## Features

- **Story-centric navigation** — sidebar tree organised by Category → Component → Story, just like Storybook.js.
- **Live props editor** — auto-generated from [`schemars::JsonSchema`](https://docs.rs/schemars); edit props in real time and see the component update instantly.
- **Decorators** — wrap stories with extra markup (padding, theme providers, etc.) via simple function pointers.
- **Documentation pages** — embed Markdown docs in the sidebar with the `storydoc!` macro, including live `@[story:...]` previews.
- **Viewport & zoom controls** — resize the preview viewport and zoom in/out to test responsive layouts.
- **Grid & outline overlays** — toggle a grid overlay or element outlines inside the preview iframe.
- **Zero-config registration** — the `#[storybook]` attribute macro and the [`inventory`](https://docs.rs/inventory) crate handle compile-time discovery automatically.

## Quick Start

### 1. Add the dependency

```sh
cargo add storybook
```

### 2. Annotate your component

```rust
use dioxus::prelude::*;
use storybook::{storybook, Stories, Story};

#[storybook(tag = "Examples")]
#[component]
pub fn MyButton(label: String, #[props(default = false)] disabled: bool) -> Element {
    rsx! { button { disabled, "{label}" } }
}
```

### 3. Implement the `Stories` trait

```rust
impl Stories for MyButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                label: "Click me".to_string(),
                disabled: false,
            }),
            Story::with_description(
                "Disabled",
                "A disabled button that cannot be clicked",
                Self { label: "Can't click".to_string(), disabled: true },
            ),
        ]
    }
}
```

### 4. Launch the storybook

```rust
fn main() {
    storybook::launch(
        storybook::StorybookConfig::default()
            .with_title("My Component Library"),
    );
}
```

Then serve with [Dioxus CLI](https://dioxuslabs.com/learn/0.6/CLI/):

```sh
dx serve --platform web
```

## Categories & Folders

The `tag` parameter on `#[storybook]` controls sidebar placement. Use `/` to
create nested folders:

```rust
#[storybook(tag = "Forms/Inputs")]
#[component]
pub fn TextInput(/* ... */) -> Element { /* ... */ }
```

This produces a sidebar tree like:

```
Forms/
  Inputs/
    TextInput
      Default
      With Placeholder
```

## Documentation Pages

There are two ways to add documentation pages to the storybook sidebar.

### Inline doc comments

Add `///` doc comments directly on your component function. The `#[storybook]`
macro extracts them, converts the Markdown to HTML, and displays a
**Documentation** link next to the component in the sidebar:

```rust,ignore
/// A reusable button.
///
/// Renders a styled `<button>` with a customizable label.
///
/// ## Examples
///
/// A default button:
///
/// @[story:Forms/MyButton/Default]
///
/// A disabled button:
///
/// @[story:Forms/MyButton/Disabled]
#[storybook(tag = "Forms")]
#[component]
pub fn MyButton(label: String, #[props(default = false)] disabled: bool) -> Element {
    rsx! { button { disabled, "{label}" } }
}
```

The `@[story:Category/Component/Story]` syntax embeds a live, interactive
story preview directly inside the documentation page.

### Standalone Markdown files

For documentation that isn't tied to a single component, use the `storydoc!`
macro to register a Markdown file:

```rust,ignore
storybook::storydoc!("Getting Started", "assets/getting-started.md");
```

The first argument is the path in the sidebar tree; the second is the path to
the Markdown file (relative to `CARGO_MANIFEST_DIR`).

`@[story:...]` embeds work in Markdown files too:

```markdown
## Button Examples

@[story:Forms/MyButton/Default]
```

## Decorators

Wrap stories with extra markup using decorator functions:

```rust,ignore
Story::new("With Padding", MyProps { /* ... */ })
    .with_decorator(|story| rsx! {
        div { style: "padding: 20px;", {story} }
    })
```

## Injecting Component CSS

If your component library has its own stylesheet, inject it into the preview
iframes so components render correctly:

```rust,ignore
const MY_CSS: Asset = asset!("assets/my-components.scss");

fn main() {
    storybook::launch(
        storybook::StorybookConfig::default()
            .with_css(MY_CSS)
            .with_title("My Component Library"),
    );
}
```

## Requirements

- **Rust** ≥ 1.85 (edition 2024)
- **Dioxus** 0.7.x
- **Platform**: Web (WASM) — served via `dx serve`

## License

MIT
