extern crate url;
extern crate isocountry;
extern crate iso_currency;
extern crate isolanguage_1;
extern crate hyper_tls;
extern crate log;
pub mod models;
pub mod chatex;
pub mod coin;
pub mod endpoint;
pub mod extractor;

pub struct ChatexClient<TConnector> {
    client: hyper::Client<TConnector>,
    chatex: chatex::Chatex,
    api_context: chatex::ApiContext,
    access_controller: AccessController,
}

impl<TConnector> ChatexClient<TConnector> 
where TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static {
    pub fn new(
        connector: TConnector,
        base_url: url::Url,
        secret: String,
    ) -> ChatexClient<TConnector> 
    where TConnector: hyper::client::connect::Connect + Clone + Send + Sync + 'static
    {
        let client = hyper::Client::builder()
            .build::<TConnector, hyper::Body>(connector);
        let base_context = chatex::BaseContext::new(base_url);
        let api_context = chatex::ApiContext::new(base_context.clone(), secret);
        let chatex = chatex::Chatex::new(
            endpoint::Profile::new(&base_context),
            endpoint::Coin::new(&base_context),
            endpoint::Exchange::new(&base_context),
            endpoint::Invoice::new(&base_context),
            endpoint::PaymentSystem::new(&base_context));
        let access_controller = AccessController::new();
        ChatexClient {
            client,
            api_context,
            chatex,
            access_controller,
        }
    }

    pub async fn get_basic_info(&self) -> Option<models::BasicInfo> {
        if let Some(access_token) = self.get_access_token().await {
            let me_request = self.chatex.profile.get_me(access_token.as_ref())
                .expect("Failed to build /me request!");
            let me_response = self.client.request(me_request)
                .await
                .ok()?;
            let me_body = me_response.into_body();
            extractor::extract_basic_info(me_body).await
        } else {
            None
        }
    }

    async fn get_access_token(&self) -> Option<chatex::AccessToken> {
        self.access_controller.get_access_token(
            &self.api_context,
            &self.chatex,
            &self.client)
            .await
    }
}

struct AccessController {
    access_context: std::cell::RefCell<Option<chatex::AccessContext>>,
}

impl AccessController {
    pub fn new() -> AccessController {
        AccessController { 
            access_context: std::cell::RefCell::new(None),
        }
    }

    pub async fn get_access_token<TConnector>(
        &self,
        api_context: &chatex::ApiContext,
        chatex: &chatex::Chatex,
        client: &hyper::Client<TConnector>,
    ) -> Option<String> 
    where TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static {
        if self.access_context.borrow().is_none() {
            let auth_request = chatex
                .profile
                .get_access_token(api_context)
                .expect("Failed to craete access_token request!");
            let auth_response = client.request(auth_request).await.unwrap();
            if auth_response.status().is_success() {
                let auth_body = auth_response.into_body();
                let access_token = extractor::extract_access_token(auth_body)
                    .await
                    .expect("Failed to read the body of access token!");
                self.access_context.replace(
                    Some(chatex::AccessContext::new(
                        api_context.base.clone(),
                        access_token.access_token)));
            }
        }
        self.access_context
            .borrow()
            .as_ref()
            .map_or_else(
                || None, 
                |access_context| Some(access_context.access_token.clone()))
    }
}
