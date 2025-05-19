use serde::{Deserialize, Serialize};

/// Error payload for endpoint responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: i32,
    pub msg: String,
}
