extern crate hyper_tls;
extern crate iso_currency;
extern crate isocountry;
extern crate isolanguage_1;
extern crate log;
extern crate url;
pub mod context;
pub mod coin;
pub mod endpoint;
pub mod extractor;
pub mod models;
pub mod error;

struct ClientBase<TConnector> {
    client: hyper::Client<TConnector>,
    api_context: context::ApiContext,
    access_controller: AccessController,
}

impl<TConnector> ClientBase<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        client: hyper::Client<TConnector>,
        api_context: context::ApiContext,
        access_controller: AccessController,
    ) -> ClientBase<TConnector> {
        ClientBase {
            client,
            api_context,
            access_controller,
        }
    }

    async fn get_access_token(&self) -> Option<context::AccessToken> {
        self.access_controller
            .get_access_token(&self.api_context, &self.client)
            .await
    }
}

pub struct ChatexClient<TConnector> {
    base: ClientBase<TConnector>,
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
        let access_controller = AccessController::new(profile.clone());
        let base = ClientBase::new(client, api_context, access_controller);
        ChatexClient {
            base,
            profile,
            coin,
            exchange,
            invoice,
            payment_system,
        }
    }

    pub fn profile(&self) -> ProfileClient<'_, TConnector> {
        ProfileClient::new(&self.base, &self.profile)
    }

    pub fn coin(&self) -> CoinClient<'_, TConnector> {
        CoinClient::new(&self.base, &self.coin)
    }

    pub fn exchange(&self) -> ExchangeClient<'_, TConnector> {
        ExchangeClient::new(&self.base, &self.exchange)
    }

    pub fn invoice(&self) -> InvoiceClient<'_, TConnector> {
        InvoiceClient::new(&self.base, &self.invoice)
    }

    pub fn payment_system(&self) -> PaymentSystemClient<'_, TConnector> {
        PaymentSystemClient::new(&self.base, &self.payment_system)
    }
}

struct AccessController {
    access_context: std::cell::RefCell<Option<context::AccessContext>>,
    profile: endpoint::Profile,
}

impl AccessController {
    pub fn new(profile: endpoint::Profile) -> AccessController {
        AccessController {
            access_context: std::cell::RefCell::new(None),
            profile,
        }
    }

    pub async fn get_access_token<TConnector>(
        &self,
        api_context: &context::ApiContext,
        client: &hyper::Client<TConnector>,
    ) -> Option<String>
    where
        TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
    {
        if self.access_context.borrow().is_none() 
            || self.access_context.borrow().as_ref().unwrap().expired() {
            log::info!("Requesting new access token!");
            let auth_request = self
                .profile
                .get_access_token(api_context)
                .expect("Failed to create access_token request!");
            let auth_response = client.request(auth_request).await.unwrap();
            if auth_response.status().is_success() {
                let auth_body = auth_response.into_body();
                let access_token = extractor::extract_access_token(auth_body)
                    .await
                    .expect("Failed to read the body of access token!");
                self.access_context.replace(Some(context::AccessContext::new(
                    api_context.base.clone(),
                    access_token,
                )));
            }
        }
        self.access_context.borrow().as_ref().map_or_else(
            || None,
            |access_context| Some(access_context.access_token.access_token.clone()),
        )
    }
}

pub struct ProfileClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    profile: &'a endpoint::Profile,
}

impl<'a, TConnector> ProfileClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        profile: &'a endpoint::Profile,
    ) -> ProfileClient<'a, TConnector> {
        ProfileClient { base, profile }
    }

    pub async fn create_access_token(&self) -> Option<models::AccessToken> {
        let auth_request = self
            .profile
            .get_access_token(&self.base.api_context)
            .expect("Failed to create access_token request!");
        let auth_response = self.base.client.request(auth_request).await.unwrap();
        if auth_response.status().is_success() {
            let auth_body = auth_response.into_body();
            extractor::extract_access_token(auth_body).await
        } else {
            None
        }
    }

    pub async fn get_account_information(&self) -> Option<models::BasicInfo> {
        if let Some(access_token) = self.base.get_access_token().await {
            let me_request = self
                .profile
                .get_me(&access_token)
                .expect("Failed to build /me request!");
            let me_response = self.base.client.request(me_request).await.ok()?;
            let me_body = me_response.into_body();
            extractor::extract_basic_info(me_body).await
        } else {
            None
        }
    }

    pub async fn get_balance_summary(&self) -> Option<models::Balance> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .profile
                .get_balance(&access_token)
                .expect("Failed to build /balance request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_balance(body).await
        } else {
            None
        }
    }
}

pub struct CoinClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    coin: &'a endpoint::Coin,
}

impl<'a, TConnector> CoinClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        coin: &'a endpoint::Coin,
    ) -> CoinClient<'a, TConnector> {
        CoinClient { base, coin }
    }

    pub async fn get_available_coins(&self) -> Option<models::Coins> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .coin
                .coins(&access_token)
                .expect("Failed to build /coins request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_coins(body).await
        } else {
            None
        }
    }

    pub async fn get_coin(&self, coin: coin::Coin) -> Option<models::Coin> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .coin
                .coin(coin, &access_token)
                .expect("Failed to build /coins/name request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_coin(body).await
        } else {
            None
        }
    }
}

pub struct ExchangeClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    exchange: &'a endpoint::Exchange,
}

impl<'a, TConnector> ExchangeClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        exchange: &'a endpoint::Exchange,
    ) -> ExchangeClient<'a, TConnector> {
        ExchangeClient { base, exchange }
    }

    pub async fn get_all_orders(
        &self,
        pair: coin::CoinPair,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Option<models::Orders> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .get_orders(pair, offset, limit, &access_token)
                .expect("Failed to build /orders request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_orders(body).await
        } else {
            None
        }
    }

    pub async fn create_order_raw(
        &self,
        pair: coin::CoinPair,
        amount: String,
        rate: String,
    ) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .post_order(pair, amount, rate, &access_token)
                .expect("Failed to build /orders request!");
            let response = self.base.client.request(request).await.ok()?;
            log::info!("Create Order response: {:#?}", response);
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn create_order(
        &self,
        pair: coin::CoinPair,
        amount: f64,
        rate: f64,
    ) -> Option<models::Order> {
        self.create_order_raw(pair, amount.to_string(), rate.to_string())
            .await
    }

    pub async fn get_my_orders(
        &self,
        pair: Option<coin::CoinPair>,
        status: Option<String>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Option<models::Orders> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .get_my_orders(pair, status, offset, limit, &access_token)
                .expect("Failed to build /orders/my request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_orders(body).await
        } else {
            None
        }
    }

    pub async fn get_trades(
        &self,
        order_id: Option<u32>,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Option<models::Trades> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .get_trades(order_id, offset, limit, &access_token)
                .expect("Failed to build /trades request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_trades(body).await
        } else {
            None
        }
    }

    pub async fn get_trade_by_id(&self, id: &str) -> Option<models::Trade> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .get_trade_by_id(id, &access_token)
                .expect("Failed to build /trades/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_trade(body).await
        } else {
            None
        }
    }

    pub async fn get_order_by_id(&self, id: &str) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .get_order_by_id(id, &access_token)
                .expect("Failed to build /orders/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn update_order_by_id(
        &self,
        id: &str,
        order: models::UpdateOrder,
    ) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .update_order_by_id(id, order, &access_token)
                .expect("Failed to build /orders/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn delete_order_by_id(&self, id: &str) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .delete_order_by_id(id, &access_token)
                .expect("Failed to build /orders/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn activate_order_by_id(&self, id: &str) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .activate_order_by_id(id, &access_token)
                .expect("Failed to build /orders/id/activate request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn deactivate_order_by_id(&self, id: &str) -> Option<models::Order> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .deactivate_order_by_id(id, &access_token)
                .expect("Failed to build /orders/id/deactivate request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_order(body).await
        } else {
            None
        }
    }

    pub async fn create_trade_for_order(
        &self,
        id: &str,
        trade: &models::CreateTradeRequest,
    ) -> Option<models::Trade> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .exchange
                .create_trade_for_order(id, trade, &access_token)
                .expect("Failed to build /orders/id/trades request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_trade(body).await
        } else {
            None
        }
    }
}

pub struct InvoiceClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    invoice: &'a endpoint::Invoice,
}

impl<'a, TConnector> InvoiceClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        invoice: &'a endpoint::Invoice,
    ) -> InvoiceClient<'a, TConnector> {
        InvoiceClient { base, invoice }
    }

    pub async fn get_invoices(
        &self,
        coins: Option<&[coin::Coin]>,
        fiat: Option<&[iso_currency::Currency]>,
        country_code: Option<&[isocountry::CountryCode]>,
        payment_system_id: Option<&[models::PaymentSystemId]>,
        lang_id: Option<&[isolanguage_1::LanguageCode]>,
        status: Option<&[models::InvoiceStatus]>,
        offset: Option<u32>,
        limit: Option<u32>,
        date_start: Option<chrono::DateTime<chrono::Utc>>,
        date_end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Option<models::Invoices> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .invoice
                .get_invoices(
                    coins,
                    fiat,
                    country_code,
                    payment_system_id,
                    lang_id,
                    status,
                    offset,
                    limit,
                    date_start,
                    date_end,
                    &access_token,
                )
                .expect("Failed to build /invoices request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_invoices(body).await
        } else {
            None
        }
    }

    pub async fn create_invoice(
        &self,
        invoice: models::CreateInvoice,
    ) -> Option<models::Invoices> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .invoice
                .create_invoice(invoice, &access_token)
                .expect("Failed to build /invoices request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_invoices(body).await
        } else {
            None
        }
    }

    pub async fn get_invoice_by_id(&self, id: String) -> Option<models::Invoice> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .invoice
                .get_invoice_by_id(id, &access_token)
                .expect("Failed to build /invoices/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_invoice(body).await
        } else {
            None
        }
    }
}

pub struct PaymentSystemClient<'a, TConnector> {
    base: &'a ClientBase<TConnector>,
    payment_system: &'a endpoint::PaymentSystem,
}

impl<'a, TConnector> PaymentSystemClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn new(
        base: &'a ClientBase<TConnector>,
        payment_system: &'a endpoint::PaymentSystem,
    ) -> PaymentSystemClient<'a, TConnector> {
        PaymentSystemClient {
            base,
            payment_system,
        }
    }

    pub async fn get_list_of_estimated_payment_systems(
        &self,
        estimate: models::Estimate,
    ) -> Option<models::FiatEstimations> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .payment_system
                .get_list_of_estimated_payment_systems(estimate, &access_token)
                .expect("Failed to build /payment-system/estimate request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_fiat_estimations(body).await
        } else {
            None
        }
    }

    pub async fn get_payment_system_by_id(
        &self,
        id: models::PaymentSystemId,
    ) -> Option<models::PaymentSystem> {
        if let Some(access_token) = self.base.get_access_token().await {
            let request = self
                .payment_system
                .get_payment_system_by_id(id, &access_token)
                .expect("Failed to build /payment-system/id request!");
            let response = self.base.client.request(request).await.ok()?;
            let body = response.into_body();
            extractor::extract_payment_system(body).await
        } else {
            None
        }
    }
}
