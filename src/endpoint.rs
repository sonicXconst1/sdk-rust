use super::chatex;
use super::coin;
use super::models;
use url;

pub struct Profile {
    auth: url::Url,
    me: url::Url,
    balance: url::Url,
}

impl Profile {
    pub fn new(base_context: &chatex::BaseContext) -> Profile {
        let mut auth = base_context.base_url.clone();
        auth.path_segments_mut()
            .unwrap()
            .push("auth")
            .push("access-token");
        let mut me = base_context.base_url.clone();
        me.path_segments_mut().unwrap().push("me");
        let mut balance = base_context.base_url.clone();
        balance
            .path_segments_mut()
            .unwrap()
            .push("me")
            .push("balance");
        Profile { auth, me, balance }
    }

    pub fn get_access_token(
        &self,
        api_context: &chatex::ApiContext,
    ) -> Option<http::Request<hyper::Body>> {
        create_default_request_builder(&api_context.api_key)
            .method(hyper::Method::POST)
            .uri(self.auth.to_string())
            .body(hyper::Body::empty())
            .ok()
    }

    pub fn get_me(
        &self,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_context.access_token, &self.me)
    }

    pub fn get_balance(
        &self,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_context.access_token, &self.balance)
    }
}

pub struct Coin {
    coins: url::Url,
}

impl Coin {
    pub fn new(base_context: &chatex::BaseContext) -> Coin {
        let mut coins = base_context.base_url.clone();
        coins.path_segments_mut().unwrap().push("coins");
        Coin { coins }
    }

    pub fn coins(
        &self,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        create_get_request_with_url(&access_context.access_token, &self.coins)
    }

    pub fn coin(
        &self,
        coin: super::coin::Coin,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        // Bad solution. There should be way to implement in without allocations.
        // The simplest way is to use somehow 'static str.
        let coin: String = coin.into();
        let mut url = self.coins.clone();
        url.path_segments_mut().unwrap().push(coin.as_ref());
        create_get_request_with_url(&access_context.access_token, &url)
    }
}

pub struct Exchange {
    orders: url::Url,
    my: url::Url,
    trades: url::Url,
}

impl Exchange {
    const EXCHANGE: &'static str = "exchange";
    const ORDERS: &'static str = "orders";
    const TRADES: &'static str = "trades";
    const MY: &'static str = "my";

    pub fn new(base_context: &chatex::BaseContext) -> Exchange {
        let mut exchange_url = base_context.base_url.clone();
        exchange_url
            .path_segments_mut()
            .unwrap()
            .push(Self::EXCHANGE);
        let mut orders = exchange_url.clone();
        orders.path_segments_mut().unwrap().push(Self::ORDERS);
        let mut my = orders.clone();
        my.path_segments_mut().unwrap().push(Self::MY);
        let mut trades = orders.clone();
        trades.path_segments_mut().unwrap().push(Self::TRADES);
        Exchange { orders, my, trades }
    }

    pub fn get_orders(
        &self,
        pair: coin::CoinPair,
        offset: Option<u32>,
        limit: Option<u32>,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let pair = String::from(pair);
        let mut orders_url = self.orders.clone();
        orders_url
            .query_pairs_mut()
            .append_pair("pair", pair.as_ref());
        let offset = if let Some(offset) = &offset {
            offset
        } else {
            &0
        };
        orders_url
            .query_pairs_mut()
            .append_pair("offset", &offset.to_string());
        let limit = if let Some(limit) = &limit { limit } else { &50 };
        orders_url
            .query_pairs_mut()
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
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let pair = String::from(pair);
        let order_request = models::OrderRequest { pair, amount, rate };
        let order_request = serde_json::to_vec(&order_request).unwrap();
        create_default_request_builder(&access_context.access_token)
            .method(hyper::Method::POST)
            .uri(self.orders.to_string())
            .header("Content-Type", "application/json")
            .body(hyper::body::Body::from(order_request))
            .ok()
    }

    pub fn get_my_orders(
        &self,
        pair: Option<coin::CoinPair>,
        status: Option<String>,
        offset: Option<u32>,
        limit: Option<u32>,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.my.clone();
        if let Some(pair) = pair {
            url.query_pairs_mut()
                .append_pair("pair", String::from(pair).as_ref());
        }
        if let Some(status) = status {
            url.query_pairs_mut().append_pair("status", status.as_ref());
        }
        Self::add_offset_and_limit_parameters(&mut url, offset, limit);
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub fn get_trades(
        &self,
        order_id: Option<u32>,
        offset: Option<u32>,
        limit: Option<u32>,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.trades.clone();
        if let Some(order_id) = order_id {
            url.query_pairs_mut()
                .append_pair("order_id", order_id.to_string().as_ref());
        }
        Self::add_offset_and_limit_parameters(&mut url, offset, limit);
        create_get_request_with_url(&access_context.access_token, &url)
    }

    pub fn get_trade_by_id(
        &self,
        id: &str,
        access_context: &chatex::AccessContext,
    ) -> Option<http::Request<hyper::Body>> {
        let mut url = self.trades.clone();
        url.path_segments_mut().unwrap().push(id);
        create_get_request_with_url(&access_context.access_token, &url)
    }

    fn add_offset_and_limit_parameters(
        url: &mut url::Url,
        offset: Option<u32>,
        limit: Option<u32>,
    ) {
        let offset = if let Some(offset) = offset { offset } else { 0 };
        let limit = if let Some(limit) = limit { limit } else { 50 };
        url.query_pairs_mut()
            .append_pair("offset", offset.to_string().as_ref())
            .append_pair("limit", limit.to_string().as_ref());
    }
}

pub struct Invoice {
    invoices: url::Url,
}

pub struct PaymentSystem {
    payment_systems: url::Url,
    estimate: url::Url,
}

fn create_default_request_builder(token: &str) -> http::request::Builder {
    http::request::Builder::new()
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &token))
}

fn create_get_request_with_url(token: &str, url: &url::Url) -> Option<http::Request<hyper::Body>> {
    create_default_request_builder(token)
        .method(hyper::Method::GET)
        .uri(url.to_string())
        .body(hyper::Body::empty())
        .ok()
}
