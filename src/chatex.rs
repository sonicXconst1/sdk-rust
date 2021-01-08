use url;
use super::endpoint;

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

pub struct AccessContext {
    pub base: BaseContext,
    pub access_token: String,
}

impl AccessContext {
    pub fn new(base: BaseContext, access_token: String) -> AccessContext {
        AccessContext { base, access_token }
    }
}

pub struct Chatex {
    pub profile: endpoint::Profile,
    pub coin: endpoint::Coin,
    pub exchange: endpoint::Exchange,
    pub invoice: endpoint::Invoice,
}

impl Chatex {
    pub fn new(
        profile: endpoint::Profile,
        coin: endpoint::Coin,
        exchange: endpoint::Exchange,
        invoice: endpoint::Invoice,
    ) -> Chatex {
        Chatex { 
            profile,
            coin,
            exchange,
            invoice,
        }
    }
}
