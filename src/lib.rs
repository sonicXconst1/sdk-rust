pub mod coin;
pub mod context;
pub mod endpoint;
pub mod error;
pub mod extractor;
pub mod models;
pub mod client_base;
pub mod profile_client;
pub mod access_controller;
pub mod coin_client;
pub mod exchange_client;
pub mod invoice_client;
pub mod payment_system_client;
pub mod chatex_client;

pub use chatex_client::ChatexClient;
pub use profile_client::ProfileClient;
pub use coin_client::CoinClient;
pub use exchange_client::ExchangeClient;
pub use invoice_client::InvoiceClient;
pub use payment_system_client::PaymentSystemClient;

#[cfg(test)]
mod test {
    struct MockConnector { }

    use tower;

    impl tower::Service<http::Request<hyper::Body>> for MockConnector {
        type Response = http::Response<hyper::Body>;
        type Error = http::Error;
        type Future = std::pin::Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

        fn poll_ready(
            &mut self,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), Self::Error>> {
            std::task::Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<hyper::Body>) -> Self::Future {
            todo!()
        }
    }
}
