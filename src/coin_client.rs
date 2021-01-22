use super::client_base;
use super::coin;
use super::endpoint;
use super::error;
use super::extractor;
use super::models;
use hyper;

pub struct CoinClient<'a, TConnector> {
    base: &'a client_base::ClientBase<TConnector>,
    coin: &'a endpoint::Coin,
}

impl<'a, TConnector> CoinClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: &'a client_base::ClientBase<TConnector>,
        coin: &'a endpoint::Coin,
    ) -> CoinClient<'a, TConnector> {
        CoinClient { base, coin }
    }

    pub async fn get_available_coins(&self) -> Result<models::Coins, error::Error> {
        let request = self.base.create_request(self.coin, |access_token, coin| {
            coin.coins(&access_token)
                .expect("Failed to build /coins request!")
        });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_coins(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_coin(&self, coin: coin::Coin) -> Result<models::Coin, error::Error> {
        let request =
            self.base
                .create_request(self.coin, |access_token, coin_endpoint| {
                    coin_endpoint
                        .coin(coin.clone(), &access_token)
                        .expect("Failed to build /coins/name request!")
                });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_coin(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }
}
