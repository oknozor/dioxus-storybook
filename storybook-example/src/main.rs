use dioxus::prelude::*;
use storybook::{Stories, Story, StorybookConfig, storybook};

/// A versatile button component for triggering actions.
///
/// `ExampleButton` renders a styled `<button>` element with customizable text
/// and an optional disabled state. It is the most fundamental interactive
/// element in a UI and serves as a good starting point for learning how
/// Dioxus Storybook works.
///
/// # Props
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `label` | `String` | — | The text displayed inside the button. |
/// | `disabled` | `bool` | `false` | When `true`, the button is grayed out and non-interactive. |
///
/// # Usage
///
/// ```rust
/// rsx! {
///     ExampleButton { label: "Save" }
///     ExampleButton { label: "Delete", disabled: true }
/// }
/// ```
///
/// # Examples
///
/// A default button with a simple label:
///
/// @[story:Examples/Buttons/ExampleButton/Default]
///
/// A disabled button — note the reduced opacity and `not-allowed` cursor:
///
/// @[story:Examples/Buttons/ExampleButton/Disabled]
///
/// Buttons stretch to fit longer labels:
///
/// @[story:Examples/Buttons/ExampleButton/Long Label]
///
/// # Edge Cases
///
/// - An empty `label` will render a button with no visible text but the
///   padding and border remain, so the button is still clickable.
/// - Setting `disabled` to `true` adds the HTML `disabled` attribute,
///   which prevents click events and applies the browser's default
///   disabled styling.
///
/// @[story:Examples/Buttons/ExampleButton/Empty Label]
#[storybook(tag = "Examples/Buttons")]
#[component]
pub fn ExampleButton(label: String, #[props(default = false)] disabled: bool) -> Element {
    rsx! {
        button {
            padding: "8px 16px",
            border_radius: "4px",
            border: "1px solid #ccc",
            cursor: if disabled { "not-allowed" } else { "pointer" },
            opacity: if disabled { "0.5" } else { "1" },
            disabled,
            "{label}"
        }
    }
}

impl Stories for ExampleButtonProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new(
                "Default",
                Self {
                    label: "Click me".to_string(),
                    disabled: false,
                },
            ),
            Story::with_description(
                "Disabled",
                "A disabled button that cannot be clicked — note the reduced opacity and not-allowed cursor",
                Self {
                    label: "Can't click".to_string(),
                    disabled: true,
                },
            ),
            Story::with_description(
                "Long Label",
                "Demonstrates how the button stretches to accommodate longer text",
                Self {
                    label: "This is a button with a very long label to test wrapping".to_string(),
                    disabled: false,
                },
            ),
            Story::with_description(
                "Empty Label",
                "Edge case: a button with an empty label string",
                Self {
                    label: String::new(),
                    disabled: false,
                },
            ),
        ]
    }
}

/// A content card with a title and body text.
///
/// `ExampleCard` displays information in a bordered, rounded container.
/// Cards are commonly used to group related content — for example, a blog
/// post summary, a product listing, or a settings panel.
///
/// # Props
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `title` | `String` | — | The heading displayed at the top of the card. |
/// | `content` | `String` | — | The body text rendered below the title. |
///
/// # Usage
///
/// ```rust
/// rsx! {
///     ExampleCard {
///         title: "Welcome",
///         content: "Thanks for visiting our site!",
///     }
/// }
/// ```
///
/// # Examples
///
/// A basic card with a short title and content:
///
/// @[story:Examples/Data Display/ExampleCard/Default]
///
/// Cards grow vertically to accommodate longer text:
///
/// @[story:Examples/Data Display/ExampleCard/Long Content]
///
/// A minimal card with very short content:
///
/// @[story:Examples/Data Display/ExampleCard/Minimal]
///
/// # Styling
///
/// The card has a fixed `max-width` of 300 px, a subtle border, and
/// rounded corners. The title uses an `<h3>` tag and the content uses
/// a `<p>` tag with muted text color.
///
/// # Edge Cases
///
/// - Very long content will cause the card to grow vertically; there is
///   no built-in truncation or scrolling.
/// - An empty `title` still renders the `<h3>` element (zero height),
///   so the content shifts upward slightly.
#[storybook(tag = "Examples/Data Display")]
#[component]
pub fn ExampleCard(title: String, content: String) -> Element {
    rsx! {
        div { style: "border: 1px solid #ddd; border-radius: 8px; padding: 16px; max-width: 300px;",
            h3 { style: "margin: 0 0 8px 0;", "{title}" }
            p { style: "margin: 0; color: #666;", "{content}" }
        }
    }
}

impl Stories for ExampleCardProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                title: "Card Title".to_string(),
                content: "This is the card content.".to_string(),
            }),
            Story::with_description(
                "Long Content",
                "A card with longer content text to show vertical growth",
                Self {
                    title: "Featured Article".to_string(),
                    content: "This is a much longer content that demonstrates how the card handles more text. The card will grow vertically to accommodate all the content without truncation.".to_string(),
                }
            ).with_decorator(|story| rsx! {
                div {
                    display: "flex",
                    justify_content: "center",
                    align_items: "center",
                    height: "100%",
                    background_color: "white", {story}
                }
            }),
            Story::with_description(
                "Minimal",
                "A card with very short title and content",
                Self {
                    title: "Hi".to_string(),
                    content: "Short.".to_string(),
                },
            ),
        ]
    }
}

/// A small status indicator badge.
///
/// `ExampleBadge` renders a compact, pill-shaped label typically used to
/// convey status information — for example, "New", "Active", "Archived",
/// or a numeric count. Badges are often placed next to headings, inside
/// table rows, or on top of icons.
///
/// # Props
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `text` | `String` | — | The label displayed inside the badge. |
/// | `variant` | `String` | `"default"` | Visual style variant. Supported values: `"default"`, `"success"`, `"warning"`, `"error"`. |
///
/// # Usage
///
/// ```rust
/// rsx! {
///     ExampleBadge { text: "New" }
///     ExampleBadge { text: "3 errors", variant: "error" }
/// }
/// ```
///
/// # Variants
///
/// - **default** — neutral gray background.
/// - **success** — green background, indicates a positive state.
/// - **warning** — amber/yellow background, indicates caution.
/// - **error** — red background, indicates a problem.
///
/// Unknown variant values fall back to the default style.
///
/// # Examples
///
/// @[story:Examples/Feedback/ExampleBadge/Default]
///
/// @[story:Examples/Feedback/ExampleBadge/Success]
///
/// @[story:Examples/Feedback/ExampleBadge/Warning]
///
/// @[story:Examples/Feedback/ExampleBadge/Error]
#[storybook(tag = "Examples/Feedback")]
#[component]
pub fn ExampleBadge(
    text: String,
    #[props(default = "default".to_string())] variant: String,
) -> Element {
    let (bg, fg) = match variant.as_str() {
        "success" => ("#dcfce7", "#166534"),
        "warning" => ("#fef9c3", "#854d0e"),
        "error" => ("#fee2e2", "#991b1b"),
        _ => ("#f3f4f6", "#374151"),
    };

    rsx! {
        span {
            display: "inline-block",
            padding: "2px 10px",
            border_radius: "9999px",
            font_size: "12px",
            font_weight: "600",
            background_color: bg,
            color: fg,
            "{text}"
        }
    }
}

impl Stories for ExampleBadgeProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Default", Self {
                text: "Badge".to_string(),
                variant: "default".to_string(),
            }),
            Story::with_description(
                "Success",
                "Green badge indicating a positive or completed state",
                Self {
                    text: "Active".to_string(),
                    variant: "success".to_string(),
                },
            ),
            Story::with_description(
                "Warning",
                "Amber badge indicating a cautionary state",
                Self {
                    text: "Pending".to_string(),
                    variant: "warning".to_string(),
                },
            ),
            Story::with_description(
                "Error",
                "Red badge indicating a problem or failure",
                Self {
                    text: "Failed".to_string(),
                    variant: "error".to_string(),
                },
            ),
        ]
    }
}

/// An alert banner for displaying important messages.
///
/// `ExampleAlert` renders a full-width banner with an icon-like prefix,
/// a title, and an optional description. Alerts are used to communicate
/// important information that requires the user's attention — for example,
/// success confirmations, error messages, or informational notices.
///
/// # Props
///
/// | Prop | Type | Default | Description |
/// |------|------|---------|-------------|
/// | `title` | `String` | — | The bold heading of the alert. |
/// | `description` | `String` | `""` | Optional body text with additional details. |
/// | `severity` | `String` | `"info"` | Controls color and icon. Values: `"info"`, `"success"`, `"warning"`, `"error"`. |
///
/// # Usage
///
/// ```rust
/// rsx! {
///     ExampleAlert {
///         title: "Saved successfully",
///         severity: "success",
///     }
///     ExampleAlert {
///         title: "Connection lost",
///         description: "Please check your network settings.",
///         severity: "error",
///     }
/// }
/// ```
///
/// # Examples
///
/// An informational alert with a description:
///
/// @[story:Examples/Feedback/ExampleAlert/Info]
///
/// A success confirmation:
///
/// @[story:Examples/Feedback/ExampleAlert/Success]
///
/// A warning about unsaved changes:
///
/// @[story:Examples/Feedback/ExampleAlert/Warning]
///
/// An error indicating a failure:
///
/// @[story:Examples/Feedback/ExampleAlert/Error]
///
/// A compact alert with only a title (no description):
///
/// @[story:Examples/Feedback/ExampleAlert/Title Only]
///
/// # Behavior
///
/// - When `description` is empty, only the title line is shown.
/// - The left border color and emoji prefix change based on `severity`.
/// - Unknown severity values fall back to the `"info"` style.
#[storybook(tag = "Examples/Feedback")]
#[component]
pub fn ExampleAlert(
    title: String,
    #[props(default)] description: String,
    #[props(default = "info".to_string())] severity: String,
) -> Element {
    let (border_color, bg, icon) = match severity.as_str() {
        "success" => ("#22c55e", "#f0fdf4", "✅"),
        "warning" => ("#eab308", "#fefce8", "⚠️"),
        "error" => ("#ef4444", "#fef2f2", "❌"),
        _ => ("#3b82f6", "#eff6ff", "ℹ️"),
    };

    rsx! {
        div {
            border_left: "4px solid {border_color}",
            background_color: bg,
            padding: "12px 16px",
            border_radius: "0 6px 6px 0",
            max_width: "480px",
            div { style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",
                span { "{icon}" }
                strong { "{title}" }
            }
            if !description.is_empty() {
                p { style: "margin: 4px 0 0 28px; color: #555; font-size: 14px;", "{description}" }
            }
        }
    }
}

impl Stories for ExampleAlertProps {
    fn stories() -> Vec<Story<Self>> {
        vec![
            Story::new("Info", Self {
                title: "Did you know?".to_string(),
                description: "You can embed stories inside markdown documentation pages.".to_string(),
                severity: "info".to_string(),
            }),
            Story::with_description(
                "Success",
                "A success alert confirming a completed action",
                Self {
                    title: "Changes saved".to_string(),
                    description: "Your settings have been updated successfully.".to_string(),
                    severity: "success".to_string(),
                },
            ),
            Story::with_description(
                "Warning",
                "A warning alert drawing attention to a potential issue",
                Self {
                    title: "Unsaved changes".to_string(),
                    description: "You have unsaved changes that will be lost if you navigate away.".to_string(),
                    severity: "warning".to_string(),
                },
            ),
            Story::with_description(
                "Error",
                "An error alert indicating a failure",
                Self {
                    title: "Upload failed".to_string(),
                    description: "The file could not be uploaded. Please try again.".to_string(),
                    severity: "error".to_string(),
                },
            ),
            Story::with_description(
                "Title Only",
                "An alert with no description — only the title line is shown",
                Self {
                    title: "Session expired".to_string(),
                    description: String::new(),
                    severity: "warning".to_string(),
                },
            ),
        ]
    }
}

storybook::storydoc!("Examples", "assets/getting-started.md");

fn main() {
    storybook::launch(StorybookConfig::default().with_title("Example Storybook"));
}