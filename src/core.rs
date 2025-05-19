use serde::{Deserialize, Serialize};

/// Error payload for endpoint responses
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: i32,
    pub msg: String,
}

/// Protocol-agnostic request trait
/// This trait is used to build 1-to-1 relationships between request and response types.
pub trait Request: Serialize {
    type Response: for<'a> Deserialize<'a>;
}
