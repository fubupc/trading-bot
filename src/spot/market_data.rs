use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use typed_builder::TypedBuilder;

use crate::core::Request;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookRequest {
    pub symbol: String,

    #[builder(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: u64,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookEntry {
    pub price: String,
    pub quantity: String,
}

impl Request for OrderBookRequest {
    type Response = OrderBookResponse;
}
