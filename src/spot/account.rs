use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::api::Request;

impl Request for AccountInfoRequest {
    const ENDPOINT: &'static str = "/api/v3/account";
    const METHOD: http::Method = http::Method::GET;
    const SECURE_TYPE: crate::api::SecureType = crate::api::SecureType::UserData;

    type Response = AccountInfoResponse;
}

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoRequest {
    pub omit_zero_balances: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfoResponse {
    maker_commission: u8,
    taker_commission: u8,
    buyer_commission: u8,
    seller_commission: u8,
    commission_rates: CommisionRates,
    can_trade: bool,
    can_withdraw: bool,
    can_deposit: bool,
    brokered: bool,
    require_self_trade_prevention: bool,
    prevent_sor: bool,
    update_time: u64,
    account_type: String,
    balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommisionRates {
    maker: Decimal,
    taker: Decimal,
    buyer: Decimal,
    seller: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    asset: String,
    free: Decimal,
    locked: Decimal,
}
