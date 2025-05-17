use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {}
