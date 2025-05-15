use http::Method;
use serde::{Deserialize, Serialize};

use crate::api::{Request, SecureType};

impl Request for NewOrderRequest {
    const ENDPOINT: &'static str = "/api/v3/order";
    const METHOD: Method = Method::POST;
    const SECURE_TYPE: SecureType = SecureType::Trade;
    type Response = NewOrderResponseACK;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: OrderSide,

    #[serde(flatten)]
    pub params: OrderParams,

    #[serde(flatten)]
    pub optional_params: NewOrderOptParams,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderOptParams {
    pub new_client_order_id: Option<String>,
    pub strategy_id: Option<u64>,
    pub strategy_type: Option<u64>,
    pub iceberg_qty: Option<f64>,
    pub new_order_resp_type: Option<NewOrderRespType>,
    pub self_trade_prevention_mode: Option<STPMode>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NewOrderRespType {
    ACK,
    RESULT,
    FULL,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum STPMode {
    None,
    ExpireMaker,
    ExpireTaker,
    ExpireBoth,
    Decrement,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderTrigger {
    StopPrice(f64),
    TrailingDelta(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum OrderParams {
    #[serde(rename_all = "camelCase")]
    Limit {
        time_in_force: TimeInForce,
        quantity: f64,
        price: f64,
    },

    #[serde(rename_all = "camelCase")]
    Market(MarketQuantity),

    #[serde(rename_all = "camelCase")]
    StopLimit {
        quantity: f64,

        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    StopLossLimit {
        time_in_force: TimeInForce,
        quantity: f64,
        price: f64,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfit {
        quantity: f64,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfitLimit {
        time_in_force: TimeInForce,
        quantity: f64,
        price: f64,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    LimitMaker { quantity: f64, price: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketQuantity {
    Quantity(f64),
    QuoteOrderQty(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderResponseACK {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64,
    pub client_order_id: String,
    pub transact_time: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_request() {
        assert_eq!(
            "symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1.0&price=100000.0",
            serde_urlencoded::to_string(NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Buy,
                params: OrderParams::Limit {
                    time_in_force: TimeInForce::GTC,
                    quantity: 1.0,
                    price: 100000.0,
                },
                optional_params: Default::default(),
            })
            .unwrap()
        );

        assert_eq!(
            "symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.0",
            serde_urlencoded::to_string(NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Sell,
                params: OrderParams::Market(MarketQuantity::Quantity(1.0)),
                optional_params: Default::default(),
            })
            .unwrap()
        );

        assert_eq!(
            "symbol=BTCUSDT&side=SELL&type=STOP_LIMIT&quantity=1.0&stopPrice=120000.0",
            serde_urlencoded::to_string(NewOrderRequest {
                symbol: "BTCUSDT".to_string(),
                side: OrderSide::Sell,
                params: OrderParams::StopLimit {
                    quantity: 1.0,
                    trigger: OrderTrigger::StopPrice(120000.0)
                },
                optional_params: Default::default(),
            })
            .unwrap()
        );
    }
}
