/// Represents a parsed section of documentation content.
#[derive(Clone, Debug)]
pub enum DocPart {
    Html(String),
    StoryEmbed {
        story_path: String,
        story_name: String,
    },
}

