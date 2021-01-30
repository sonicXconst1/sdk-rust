use super::client_base;
use super::coin;
use super::endpoint;
use super::error;
use super::extractor;
use super::models;
use hyper;

pub struct ExchangeClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    exchange: std::sync::Arc<endpoint::Exchange>,
}

impl<TConnector> ExchangeClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: std::sync::Arc<client_base::ClientBase<TConnector>>,
        exchange: std::sync::Arc<endpoint::Exchange>,
    ) -> ExchangeClient<TConnector> {
        ExchangeClient { base, exchange }
    }

    pub async fn get_all_orders(
        &self,
        pair: coin::CoinPair,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<models::Orders, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .get_orders(pair.clone(), offset, limit, &access_token)
                        .expect("Failed to build /orders request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_orders(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_order_raw(
        &self,
        pair: coin::CoinPair,
        amount: &str,
        rate: &str,
    ) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .post_order(
                            pair.clone(),
                            amount.into(),
                            rate.into(),
                            &access_token,
                        )
                        .expect("Failed to build /orders request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_order(
        &self,
        pair: coin::CoinPair,
        amount: f64,
        rate: f64,
    ) -> Result<models::Order, error::Error> {
        self.create_order_raw(pair, &amount.to_string(), &rate.to_string())
            .await
    }

    pub async fn get_my_orders(
        &self,
        pair: Option<coin::CoinPair>,
        status: Option<String>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<models::Orders, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .get_my_orders(
                            pair.clone(),
                            status.clone(),
                            offset,
                            limit,
                            &access_token,
                        )
                        .expect("Failed to build /orders/my request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_orders(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_trades(
        &self,
        order_id: Option<u32>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<models::Trades, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .get_trades(order_id, offset, limit, &access_token)
                        .expect("Failed to build /trades request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_trades(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_trade_by_id(&self, id: &str) -> Result<models::Trade, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .get_trade_by_id(id, &access_token)
                        .expect("Failed to build /trades/id request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_trade(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_order_by_id(&self, id: &str) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .get_order_by_id(id, &access_token)
                        .expect("Failed to build /orders/id request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn update_order_by_id(
        &self,
        id: &str,
        order: &models::UpdateOrder,
    ) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .update_order_by_id(id, order.clone(), &access_token)
                        .expect("Failed to build /orders/id request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn delete_order_by_id(
        &self,
        id: &str,
    ) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .delete_order_by_id(id, &access_token)
                        .expect("Failed to build /orders/id request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn activate_order_by_id(
        &self,
        id: &str,
    ) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .activate_order_by_id(id, &access_token)
                        .expect("Failed to build /orders/id/activate request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn deactivate_order_by_id(
        &self,
        id: &str,
    ) -> Result<models::Order, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .deactivate_order_by_id(id, &access_token)
                        .expect("Failed to build /orders/id/activate request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_order(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn create_trade_for_order(
        &self,
        id: &str,
        trade: &models::CreateTradeRequest,
    ) -> Result<models::Trade, error::Error> {
        let request =
            self.base
                .create_request(self.exchange.as_ref(), |access_token, exchange| {
                    exchange
                        .create_trade_for_order(id, trade, &access_token)
                        .expect("Failed to build /orders/id/trade request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_trade(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }
}
