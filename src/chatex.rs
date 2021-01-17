use url;
use super::models;
use chrono;

#[derive(Clone)]
pub struct BaseContext {
    pub base_url: url::Url
}

impl BaseContext {
    pub fn new(base_url: url::Url) -> BaseContext {
        BaseContext { 
            base_url 
        }
    }
}

pub struct ApiContext {
    pub base: BaseContext,
    pub api_key: String,
}

impl ApiContext {
    pub fn new(base: BaseContext, api_key: String) -> ApiContext {
        ApiContext { base, api_key }
    }
}

pub type AccessToken = String;

pub struct AccessContext {
    pub base: BaseContext,
    pub access_token: models::AccessToken,
    expires_at: chrono::DateTime::<chrono::Utc>,
}

impl AccessContext {
    const TIME_EXPIRATION_TOLERANCE: i64 = 60;

    pub fn new(base: BaseContext, access_token: models::AccessToken) -> AccessContext {
        let expires_at = chrono::NaiveDateTime::from_timestamp(
            access_token.expires_at,
            0);
        let expires_at = chrono::DateTime::<chrono::Utc>::from_utc(
            expires_at,
            chrono::Utc);
        AccessContext { 
            base,
            access_token,
            expires_at,
        }
    }

    pub fn expired(&self) -> bool {
        let subtracted = chrono::Utc::now().signed_duration_since(self.expires_at);
        subtracted.num_seconds() > -Self::TIME_EXPIRATION_TOLERANCE
    }

    pub fn not_expired(&self) -> bool {
        !self.expired()
    }
}
