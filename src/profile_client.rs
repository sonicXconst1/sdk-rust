use super::client_base;
use super::endpoint;
use super::error;
use super::extractor;
use super::models;
use hyper;

pub struct ProfileClient<TConnector> {
    base: std::sync::Arc<client_base::ClientBase<TConnector>>,
    profile: std::sync::Arc<endpoint::Profile>,
}

impl<TConnector> ProfileClient<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: std::sync::Arc<client_base::ClientBase<TConnector>>,
        profile: std::sync::Arc<endpoint::Profile>,
    ) -> ProfileClient<TConnector> {
        ProfileClient { base, profile }
    }

    pub async fn create_access_token(&self) -> Result<models::AccessToken, error::Error> {
        let auth_request = self
            .profile
            .get_access_token(&self.base.api_context)
            .expect("Failed to create access_token request!");
        let auth_response = self.base.client.request(auth_request).await.unwrap();
        if error::Error::is_error_code(auth_response.status()) {
            let error =
                error::Error::to_error(auth_response.status(), auth_response.into_body()).await;
            Err(error)
        } else {
            let auth_body = auth_response.into_body();
            Ok(extractor::extract_access_token(auth_body).await.unwrap())
        }
    }

    pub async fn get_account_information(&self) -> Result<models::BasicInfo, error::Error> {
        let request = self
            .base
            .create_request(self.profile.as_ref(), |access_token, profile| {
                profile
                    .get_me(&access_token)
                    .expect("Failed to build /me request")
            });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_basic_info(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_balance_summary(&self) -> Result<models::Balance, error::Error> {
        let request = self
            .base
            .create_request(self.profile.as_ref(), |access_token, profile| {
                profile
                    .get_balance(&access_token)
                    .expect("Failed to build /balance request")
            });
        match request.await {
            Ok(request) => {
                self.base
                    .call_to_endpoint(request, |body| extractor::extract_balance(body))
                    .await
            }
            Err(error) => Err(error),
        }
    }
}
