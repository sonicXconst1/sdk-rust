use super::client_base;
use super::endpoint;
use super::error;
use super::extractor;
use super::models;
use hyper;

pub struct ProfileClient<'a, TConnector> {
    base: &'a client_base::ClientBase<TConnector>,
    profile: &'a endpoint::Profile,
}

impl<'a, TConnector> ProfileClient<'a, TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(
        base: &'a client_base::ClientBase<TConnector>,
        profile: &'a endpoint::Profile,
    ) -> ProfileClient<'a, TConnector> {
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
                error::Error::to_error(auth_response.status(), auth_response.into_body())
                    .await;
            Err(error)
        } else {
            let auth_body = auth_response.into_body();
            Ok(extractor::extract_access_token(auth_body).await.unwrap())
        }
    }

    pub async fn get_account_information(
        &self,
    ) -> Result<models::BasicInfo, error::Error> {
        let access_token = match self.base.get_access_token().await {
            Ok(access_token) => access_token,
            Err(error) => return Err(error),
        };
        let me_request = self
            .profile
            .get_me(&access_token)
            .expect("Failed to build /me request");
        match self.base.client.request(me_request).await {
            Ok(me_response) => {
                if error::Error::is_error_code(me_response.status()) {
                    let error = error::Error::to_error(
                        me_response.status(),
                        me_response.into_body(),
                    )
                    .await;
                    Err(error)
                } else {
                    let me_body = me_response.into_body();
                    Ok(extractor::extract_basic_info(me_body).await.unwrap())
                }
            }
            Err(error) => {
                log::error!("{}", error);
                Err(error::Error::InternalServerError)
            }
        }
    }

    pub async fn get_balance_summary(&self) -> Result<models::Balance, error::Error> {
        let access_token = match self.base.get_access_token().await {
            Ok(access_token) => access_token,
            Err(error) => return Err(error),
        };
        let request = self
            .profile
            .get_balance(&access_token)
            .expect("Failed to build /balance request");
        match self.base.client.request(request).await {
            Ok(response) => {
                if error::Error::is_error_code(response.status()) {
                    let body = response.into_body();
                    Ok(extractor::extract_balance(body).await.unwrap())
                } else {
                    let error =
                        error::Error::to_error(response.status(), response.into_body())
                            .await;
                    Err(error)
                }
            }
            Err(error) => {
                log::error!("{}", error);
                Err(error::Error::InternalServerError)
            }
        }
    }
}
