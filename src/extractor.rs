use serde;
use serde_json;
use super::models;
use hyper;

pub async fn extract_access_token(
    body: hyper::Body) -> Option<models::AccessToken> {
    read_body::<models::AccessToken>(body)
        .await
}

pub async fn extract_basic_info(body: hyper::Body) -> Option<models::BasicInfo> {
    read_body::<models::BasicInfo>(body)
        .await
}

pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
    read_body::<models::Balance>(body)
        .await
}

pub async fn extract_coins(body: hyper::Body) -> Option<models::Coins> {
    read_body::<models::Coins>(body)
        .await
}

pub async fn extract_coin(body: hyper::Body) -> Option<models::Coin> {
    read_body::<models::Coin>(body)
        .await
}

pub async fn extract_orders(body: hyper::Body) -> Option<models::Orders> {
    read_body::<models::Orders>(body)
        .await
}

pub async fn extract_trades(body: hyper::Body) -> Option<models::Trades> {
    read_body::<models::Trades>(body)
        .await
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
