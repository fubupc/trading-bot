use rust_decimal::prelude::*;

use trading_bot::{
    api::{self, API},
    spot::{
        account::{AccountInfoRequest, AccountInfoResponse},
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

    for balance in &account_info.balances {
        println!(
            "Asset: {:16} Free: {:16} Locked: {:16}",
            balance.asset, balance.free, balance.locked
        );
    }

    // create_new_order(&api).await;
}

async fn create_new_order(api: &API) {
    let new_order_req = NewOrderRequest::builder()
        .symbol("BTCUSDT".to_owned())
        .side(OrderSide::Buy)
        .params(OrderParams::Limit {
            time_in_force: TimeInForce::GTC,
            quantity: dec!(1.0),
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
