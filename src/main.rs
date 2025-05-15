use rust_decimal::prelude::*;

use trading_bot::{
    api::{self, API},
    spot::{
        account::{AccountInfoRequest, AccountInfoResponse},
        market_data::{OrderBookRequest, OrderBookResponse},
        trading::{NewOrderRequest, OrderParams, OrderSide, TimeInForce},
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let api_key = std::env::var("API_KEY").expect("API_KEY not set");
    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let api_base_url = std::env::var("API_BASE_URL").expect("API_BASE_URL not set");

    log::info!("API Key: {}", api_key);
    log::info!("Secret Key: {}", secret_key);
    log::info!("API Base URL: {}", api_base_url);

    let api = API::new(&api_key, &secret_key, &api_base_url).unwrap();
    let account_info = get_account_info(&api)
        .await
        .expect("Failed to get account info");

    println!("{:^16}{:^16}{:^16}", "Asset", "Free", "Locked");
    for balance in &account_info.balances[..10] {
        println!(
            "{:-16}{:>16}{:>16}",
            balance.asset, balance.free, balance.locked
        );
    }

    let order_book = get_order_book(&api)
        .await
        .expect("Failed to get order book");

    println!("{:^16}{:^16}", "Ask Price", "Quantity");
    for ask in order_book.asks[..10].iter().rev() {
        println!("{:>16}{:>16}", ask.price, ask.quantity);
    }

    println!("{:^16}{:^16}", "Bid Price", "Quantity");
    for bid in order_book.bids[..10].iter() {
        println!("{:>16}{:>16}", bid.price, bid.quantity);
    }

    // create_new_order(&api).await;
}

async fn create_new_order(api: &API) {
    let new_order_req = NewOrderRequest::builder()
        .symbol("BTCUSDT".to_owned())
        .side(OrderSide::Buy)
        .params(OrderParams::Limit {
            time_in_force: TimeInForce::GTC,
            quantity: dec!(0.01),
            price: dec!(100000.0),
        })
        .build();

    let resp = api
        .send(new_order_req)
        .await
        .expect("Failed to send new order request");

    println!("Response for new order request: {:?}", resp);
}

async fn get_account_info(api: &API) -> Result<AccountInfoResponse, api::Error> {
    let account_info_req = AccountInfoRequest::builder()
        .omit_zero_balances(Some(true))
        .build();
    api.send(account_info_req).await
}

async fn get_order_book(api: &API) -> Result<OrderBookResponse, api::Error> {
    let order_book_req = OrderBookRequest::builder()
        .symbol("BTCUSDT".to_string())
        .build();
    api.send(order_book_req).await
}
