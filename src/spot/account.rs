use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::core::Request;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoRequest {
    pub omit_zero_balances: Option<bool>,
}

impl Request for AccountInfoRequest {
    type Response = AccountInfoResponse;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    pub maker_commission: u8,
    pub taker_commission: u8,
    pub buyer_commission: u8,
    pub seller_commission: u8,
    pub commission_rates: CommisionRates,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub require_self_trade_prevention: bool,
    pub prevent_sor: bool,
    pub update_time: u64,
    pub account_type: String,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommisionRates {
    pub maker: Decimal,
    pub taker: Decimal,
    pub buyer: Decimal,
    pub seller: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}
