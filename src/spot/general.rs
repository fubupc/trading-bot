use serde::{Deserialize, Serialize};

use crate::core::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {}

impl Request for PingRequest {
    type Response = PingResponse;
}
