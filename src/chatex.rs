use url;

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
    pub access_token: AccessToken,
}

impl AccessContext {
    pub fn new(base: BaseContext, access_token: String) -> AccessContext {
        AccessContext { base, access_token }
    }
}
