use trading_bot::{
    api::API,
    spot::trading::{NewOrderRequest, OrderParams, OrderSide, TimeInForce},
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

    let new_order = NewOrderRequest::builder()
        .symbol("BTCUSDT".to_owned())
        .side(OrderSide::Buy)
        .params(OrderParams::Limit {
            time_in_force: TimeInForce::GTC,
            quantity: 1.0,
            price: 100000.0,
        })
        .build();

    let resp = api.send(new_order).await.expect("Failed to send request");

    println!("Response: {:?}", resp);
}
