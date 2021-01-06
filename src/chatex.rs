use http;
use hyper;
use url;
use super::models;
use super::coin;

#[derive(Clone)]
pub struct BaseContext {
    base_url: url::Url
}

impl BaseContext {
    pub fn new(base_url: url::Url) -> BaseContext {
        BaseContext { 
            base_url 
        }
    }
}

pub struct ApiContext {
    base: BaseContext,
    api_key: String,
}

impl ApiContext {
    pub fn new(base: BaseContext, api_key: String) -> ApiContext {
        ApiContext { base, api_key }
    }
}

pub struct AccessContext {
    base: BaseContext,
    access_token: String,
}

impl AccessContext {
    pub fn new(base: BaseContext, access_token: String) -> AccessContext {
        AccessContext { base, access_token }
    }
}

pub struct Chatex {
    pub auth: Auth,
    pub me: Me,
    pub coin: Coin,
    pub exchange: Exchange,
}

impl Chatex {
    pub fn new(
        auth: Auth,
        me: Me,
        coin: Coin,
        exchange: Exchange,
    ) -> Chatex {
        Chatex { 
            auth,
            me,
            coin,
            exchange,
        }
    }
}

pub struct Auth {
    root: String,
    access_token: String,
}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            root: String::from("auth"),
            access_token: String::from("access-token"),
        }
    }

    pub fn access_token(
        &self,
        context: &ApiContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = context.base.base_url.clone();
        url.path_segments_mut()
            .unwrap()
            .push(self.root.as_ref())
            .push(self.access_token.as_ref());
        create_default_request_builder(&context.api_key)
            .method(hyper::Method::POST)
            .uri(url.to_string())
            .body(hyper::Body::empty())
            .ok()
    }

    pub async fn extract_access_token(
        body: hyper::Body) -> Option<models::AccessToken> {
        read_body::<models::AccessToken>(body)
            .await
    }
}

pub struct Me {
    root: String,
    balance: String,
}

impl Me {
    pub fn new() -> Me {
        Me {
            root: String::from("me"),
            balance: String::from("balance"),
        }
    }

    pub fn root(
        &self,
        access_context: &AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = access_context.base.base_url.clone();
        url.path_segments_mut()
            .unwrap()
            .push(self.root.as_ref());
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub fn balance(
        &self,
        access_context: &AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = access_context.base.base_url.clone();
        url.path_segments_mut()
            .unwrap()
            .push(self.root.as_ref())
            .push(self.balance.as_ref());
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub async fn extract_basic_info(body: hyper::Body) -> Option<models::BasicInfo> {
        read_body::<models::BasicInfo>(body)
            .await
    }

    pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
        read_body::<models::Balance>(body)
            .await
    }
}

pub struct Coin {
    coins: String,
}

impl Coin {
    pub fn new() -> Coin {
        Coin {
            coins: String::from("coins"),
        }
    }

    pub fn coins(
        &self,
        access_context: &AccessContext,
    ) ->  Option<http::Request<hyper::Body>> {
        let mut url = access_context.base.base_url.clone();
        url.path_segments_mut()
            .unwrap()
            .push(self.coins.as_ref());
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub fn coin(
        &self,
        coin_name: super::coin::Coin,
        access_context: &AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        // Bad solution. There should be way to implement in without allocations.
        let coin_name: String = coin_name.into();
        let mut url = access_context.base.base_url.clone();
        url.path_segments_mut()
            .unwrap()
            .push(self.coins.as_ref())
            .push(coin_name.as_ref());
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub async fn extract_coins(body: hyper::Body) -> Option<models::Coins> {
        read_body::<models::Coins>(body)
            .await
    }

    pub async fn extract_coin(body: hyper::Body) -> Option<models::Coin> {
        read_body::<models::Coin>(body)
            .await
    }
}

pub struct Exchange {
    root: String,
    orders: String,
    my: String,
    trades: String,
    activate: String,
    deactivate: String,
}

impl Exchange {
    pub fn new() -> Exchange {
        Exchange {
            root: "exchange".to_owned(),
            orders: "orders".to_owned(),
            my: "my".to_owned(),
            trades: "trades".to_owned(),
            activate: "activate".to_owned(),
            deactivate: "deactivate".to_owned(),
        }
    }

    pub fn get_orders(
        &self,
        pair: coin::CoinPair,
        offset: Option<u32>,
        limit: Option<u32>,
        access_context: &AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut orders_url = self.create_get_orders_uri(
            access_context.base.base_url.clone());
        let pair_string = String::from(pair);
        orders_url.query_pairs_mut()
            .append_pair("pair", pair_string.as_ref());
        let offset = if let Some(offset) = &offset {
            offset
        } else {
            &0
        };
        orders_url.query_pairs_mut()
            .append_pair("offset", &offset.to_string());
        let limit = if let Some(limit) = &limit {
            limit
        } else {
            &50
        };
        orders_url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());
        let orders_url = orders_url.into_string();
        create_default_request_builder(&access_context.access_token)
            .method(hyper::Method::GET)
            .uri(orders_url)
            .body(hyper::Body::empty())
            .ok()
    }

    pub fn post_order(
        &self,
        pair: coin::CoinPair,
        amount: String,
        rate: String,
        access_context: &AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let orders_url = self.create_get_orders_uri(
            access_context.base.base_url.clone());
        let pair = String::from(pair);
        let order_request = models::OrderRequest {
            pair,
            amount,
            rate,
        };
        let order_request = serde_json::to_vec(&order_request).unwrap();
        create_default_request_builder(&access_context.access_token)
            .method(hyper::Method::POST)
            .uri(orders_url.to_string())
            .header("Content-Type", "application/json")
            .body(hyper::body::Body::from(order_request))
            .ok()
    }

    pub async fn extract_orders(body: hyper::Body) -> Option<models::Orders> {
        read_body::<models::Orders>(body)
            .await
    }

    fn create_get_orders_uri(&self, base_url: url::Url) -> url::Url {
        let mut orders_url = url::Url::from(base_url);
        orders_url.path_segments_mut()
            .unwrap()
            .push(self.root.as_ref())
            .push(self.orders.as_ref());
        orders_url
    }
}

fn create_default_request_builder(token: &str) -> http::request::Builder {
    http::request::Builder::new()
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &token))
}

fn create_get_request_with_url(
    token: &str,
    url: &url::Url,
) -> Option<http::Request<hyper::Body>> {
    create_default_request_builder(token)
        .method(hyper::Method::GET)
        .uri(url.to_string())
        .body(hyper::Body::empty())
        .ok()
}

async fn read_body<TResult>(body: hyper::body::Body) -> Option<TResult>
where
    TResult: serde::de::DeserializeOwned,
{
    let body = hyper::body::to_bytes(body).await.ok()?;
    match serde_json::from_slice(&body) {
        Ok(result) => Some(result),
        Err(error) => {
            eprintln!("Error on read_body: {:?}", error);
            None
        }
    }
}
