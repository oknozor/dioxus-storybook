use storybook::{STORYBOOK_CSS, StorybookConfig};

fn main() {
    storybook::launch(
        StorybookConfig::default()
            .with_css(STORYBOOK_CSS)
            .with_title("Dioxus Storybook Storybook"),
    );
}
