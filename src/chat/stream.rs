use crate::StreamChoice;

use serde::Deserialize;

// Response stream
#[derive(Debug, Clone, Deserialize)]
pub struct Stream {
    pub choices: Vec<StreamChoice>,
}
