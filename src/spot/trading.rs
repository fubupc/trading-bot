use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderRequest {
    pub symbol: String,

    pub side: OrderSide,

    #[serde(flatten)]
    pub params: OrderParams,

    #[builder(default)]
    pub new_client_order_id: Option<String>,

    #[builder(default)]
    pub strategy_id: Option<u64>,

    #[builder(default)]
    pub strategy_type: Option<u64>,

    #[builder(default)]
    pub iceberg_qty: Option<Decimal>,

    #[builder(default)]
    pub new_order_resp_type: Option<NewOrderRespType>,

    #[builder(default)]
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
    StopPrice(Decimal),
    TrailingDelta(u64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum OrderParams {
    #[serde(rename_all = "camelCase")]
    Limit {
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    Market(MarketQuantity),

    #[serde(rename_all = "camelCase")]
    StopLimit {
        quantity: Decimal,

        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    StopLossLimit {
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfit {
        quantity: Decimal,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfitLimit {
        time_in_force: TimeInForce,
        quantity: Decimal,
        price: Decimal,
        #[serde(flatten)]
        trigger: OrderTrigger,
    },

    #[serde(rename_all = "camelCase")]
    LimitMaker { quantity: Decimal, price: Decimal },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MarketQuantity {
    Quantity(Decimal),
    QuoteOrderQty(Decimal),
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
    fn test_new_order_request_serialization() {
        assert_eq!(
            "symbol=BTCUSDT&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1.0&price=100000.0",
            serde_urlencoded::to_string(
                NewOrderRequest::builder()
                    .symbol("BTCUSDT".to_string())
                    .side(OrderSide::Buy)
                    .params(OrderParams::Limit {
                        time_in_force: TimeInForce::GTC,
                        quantity: dec!(1.0),
                        price: dec!(100000.0),
                    })
                    .build()
            )
            .unwrap()
        );

        assert_eq!(
            "symbol=BTCUSDT&side=SELL&type=MARKET&quantity=1.0",
            serde_urlencoded::to_string(
                NewOrderRequest::builder()
                    .symbol("BTCUSDT".to_string())
                    .side(OrderSide::Sell)
                    .params(OrderParams::Market(MarketQuantity::Quantity(dec!(1.0))))
                    .build()
            )
            .unwrap()
        );

        assert_eq!(
            "symbol=BTCUSDT&side=SELL&type=STOP_LIMIT&quantity=1.0&stopPrice=120000.0",
            serde_urlencoded::to_string(
                NewOrderRequest::builder()
                    .symbol("BTCUSDT".to_string())
                    .side(OrderSide::Sell)
                    .params(OrderParams::StopLimit {
                        quantity: dec!(1.0),
                        trigger: OrderTrigger::StopPrice(dec!(120000.0)),
                    })
                    .build()
            )
            .unwrap()
        );
    }
}
