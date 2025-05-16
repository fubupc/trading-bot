use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{Request, SecureType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ping;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl Request for Ping {
    const ENDPOINT: &'static str = "/api/v3/ping";

    const METHOD: http::Method = Method::GET;

    const SECURE_TYPE: SecureType = SecureType::None;

    type Response = EmptyResponse;
}
