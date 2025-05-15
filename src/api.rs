use hex::ToHex;
use hmac::{Hmac, Mac};
use http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::fmt::Debug;

pub struct API {
    api_key: HeaderValue,
    secret_key: Hmac<Sha256>,
    api_base_url: url::Url,
    cli: reqwest::Client,
}

pub trait Request: Serialize {
    const ENDPOINT: &'static str;
    const METHOD: Method;
    const SECURE_TYPE: SecureType;

    type Response: for<'a> Deserialize<'a>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecureType {
    None,
    Trade,
    UserData,
    UserStream,
}

#[derive(Debug)]
pub enum Error {
    Other(Box<dyn std::error::Error>),
    Network(reqwest::Error),
    API(ErrorResponse),
    ResponseParse(reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

impl API {
    pub fn new(api_key: &str, secret_key: &str, api_base_url: &str) -> Result<Self, Error> {
        let api_base_url = url::Url::parse(&api_base_url).map_err(|e| Error::Other(Box::new(e)))?;
        let api_key = HeaderValue::from_str(api_key).map_err(|e| Error::Other(Box::new(e)))?;
        let secret_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
            .map_err(|e| Error::Other(Box::new(e)))?;

        Ok(Self {
            api_key,
            secret_key,
            api_base_url,
            cli: reqwest::Client::new(),
        })
    }

    pub async fn send<T: Request + Debug>(&self, req: T) -> Result<T::Response, Error> {
        let req = self.build_request(req)?;

        let resp = self.cli.execute(req).await.map_err(Error::Network)?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            let error_response: ErrorResponse = resp.json().await.map_err(Error::ResponseParse)?;
            return Err(Error::API(error_response));
        }
        resp.json().await.map_err(|e| Error::Other(e.into()))
    }

    fn build_request<T: Request + Debug>(&self, req: T) -> Result<reqwest::Request, Error> {
        let url = self
            .api_base_url
            .join(T::ENDPOINT)
            .map_err(|e| Error::Other(Box::new(e)))?;
        let params = serde_urlencoded::to_string(&req)
            .unwrap_or_else(|_| panic!("Failed to serialize request for {:?}", req));

        let mut req = reqwest::Request::new(T::METHOD, url);
        if params != "" {
            req.url_mut().set_query(Some(&params));
        }

        match T::SECURE_TYPE {
            SecureType::None => {}
            SecureType::Trade | SecureType::UserData => {
                req.url_mut().query_pairs_mut().append_pair(
                    "timestamp",
                    &chrono::Utc::now().timestamp_millis().to_string(),
                );

                let query_string = req.url().query().unwrap_or("");
                log::debug!("query string: {}", query_string);
                let signature = Self::sign(&self.secret_key, query_string);
                log::debug!("signature: {}", signature);

                req.url_mut()
                    .query_pairs_mut()
                    .append_pair("signature", &signature);

                req.headers_mut()
                    .append("X-MBX-APIKEY", self.api_key.clone());
            }
            SecureType::UserStream => {
                req.headers_mut()
                    .append("X-MBX-APIKEY", self.api_key.clone());
            }
        };

        Ok(req)
    }

    fn sign(secret_key: &Hmac<Sha256>, query_string: &str) -> String {
        let mut mac = secret_key.clone();
        mac.update(query_string.as_bytes());
        mac.finalize().into_bytes().encode_hex()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spot::{NewOrderRequest, OrderParams, OrderSide, TimeInForce};

    #[test]
    fn test_sign() {
        let key = "NhqPtmdSJYdKjVHjA7PZj4Mge3R5YNiP1e3UZjInClVN65XAbvqqM6A7H5fATj0j";
        let data = "symbol=LTCBTC&side=BUY&type=LIMIT&timeInForce=GTC&quantity=1&price=0.1&recvWindow=5000&timestamp=1499827319559";
        let expected_signature = "c8db56825ae71d6d79447849e617115f4a920fa2acdcab2b053c4b2838bd6b71";

        let hmac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
        let signature = API::sign(&hmac, data);
        assert_eq!(signature, expected_signature);
    }

    #[test]
    fn test_build_request() {
        let api = API::new(
            "test_api_key",
            "test_secret_key",
            "https://testnet.binance.vision",
        )
        .unwrap();
        let new_order = NewOrderRequest {
            symbol: "BTCUSDT".to_owned(),
            side: OrderSide::Buy,
            params: OrderParams::Limit {
                time_in_force: TimeInForce::GTC,
                quantity: 1.0,
                price: 100000.0,
            },
            optional_params: Default::default(),
        };
        let req = api.build_request(new_order).unwrap();

        assert_eq!(req.method(), Method::POST);
        assert_eq!(req.url().path(), "/api/v3/order");
        assert_eq!(req.url().query_pairs().count(), 9);
        assert_eq!(req.headers().get("X-MBX-APIKEY").unwrap(), "test_api_key");
    }
}
