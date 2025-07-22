use serde::Deserialize;

// Response stream delta
#[derive(Debug, Clone, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
}
