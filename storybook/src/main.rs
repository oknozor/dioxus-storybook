use storybook::{StorybookConfig, STORYBOOK_CSS};

fn main() {
    storybook::launch(StorybookConfig::default()
        .with_css(STORYBOOK_CSS)
        .with_title("Dioxus Storybook Storybook"));
}
