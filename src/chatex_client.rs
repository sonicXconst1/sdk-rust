use super::{
    access_controller, client_base, coin_client, context, endpoint, exchange_client,
    invoice_client, payment_system_client, profile_client,
};
use hyper;

pub struct ChatexClient<TConnector> {
    base: client_base::ClientBase<TConnector>,
    profile: endpoint::Profile,
    coin: endpoint::Coin,
    exchange: endpoint::Exchange,
    invoice: endpoint::Invoice,
    payment_system: endpoint::PaymentSystem,
}

impl<TConnector> ChatexClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    pub fn new(
        connector: TConnector,
        base_url: url::Url,
        secret: String,
    ) -> ChatexClient<TConnector>
    where
        TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    {
        let client = hyper::Client::builder().build::<TConnector, hyper::Body>(connector);
        let base_context = context::BaseContext::new(base_url);
        let api_context = context::ApiContext::new(base_context.clone(), secret);
        let profile = endpoint::Profile::new(&base_context);
        let coin = endpoint::Coin::new(&base_context);
        let exchange = endpoint::Exchange::new(&base_context);
        let invoice = endpoint::Invoice::new(&base_context);
        let payment_system = endpoint::PaymentSystem::new(&base_context);
        let access_controller = access_controller::AccessController::new(profile.clone());
        let base = client_base::ClientBase::new(client, api_context, access_controller);
        ChatexClient {
            base,
            profile,
            coin,
            exchange,
            invoice,
            payment_system,
        }
    }

    pub fn profile(&self) -> profile_client::ProfileClient<'_, TConnector> {
        profile_client::ProfileClient::new(&self.base, &self.profile)
    }

    pub fn coin(&self) -> coin_client::CoinClient<'_, TConnector> {
        coin_client::CoinClient::new(&self.base, &self.coin)
    }

    pub fn exchange(&self) -> exchange_client::ExchangeClient<'_, TConnector> {
        exchange_client::ExchangeClient::new(&self.base, &self.exchange)
    }

    pub fn invoice(&self) -> invoice_client::InvoiceClient<'_, TConnector> {
        invoice_client::InvoiceClient::new(&self.base, &self.invoice)
    }

    pub fn payment_system(
        &self,
    ) -> payment_system_client::PaymentSystemClient<'_, TConnector> {
        payment_system_client::PaymentSystemClient::new(&self.base, &self.payment_system)
    }
}
