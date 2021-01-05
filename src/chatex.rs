use http;
use hyper;
use super::models;

#[derive(Clone)]
pub struct BaseContext {
    base_uri: String,
}

impl BaseContext {
    pub fn new(base_uri: String) -> BaseContext {
        BaseContext { base_uri }
    }
}

pub struct ApiContext {
    base: BaseContext,
    api_key: String,
}

impl ApiContext {
    pub fn new(base: BaseContext, api_key: String) -> ApiContext {
        ApiContext { base, api_key }
    }
}

pub struct AccessContext {
    base: BaseContext,
    access_token: String,
}

impl AccessContext {
    pub fn new(base: BaseContext, access_token: String) -> AccessContext {
        AccessContext { base, access_token }
    }
}

pub struct Chatex {
    pub auth: Auth,
    pub me: Me,
}

impl Chatex {
    pub fn new(auth: Auth, me: Me) -> Chatex {
        Chatex { auth, me }
    }
}

pub struct Auth {
    root: String,
    access_token: String,
}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            root: String::from("auth"),
            access_token: String::from("access-token"),
        }
    }

    pub fn access_token(
        &self,
        context: &ApiContext,
    ) -> Option<http::request::Request<hyper::Body>> {
        let uri = format!(
            "{}{}/{}",
            context.base.base_uri, self.root, self.access_token
        );
        create_default_request_builder(&context.api_key)
            .method(hyper::Method::POST)
            .uri(uri)
            .body(hyper::Body::empty())
            .ok()
    }

    pub async fn extract_access_token(
        body: hyper::Body) -> Option<models::AccessToken> {
        read_body::<models::AccessToken>(body)
            .await
    }
}

pub struct Me {
    root: String,
    balance: String,
}

impl Me {
    pub fn new() -> Me {
        Me {
            root: String::from("me"),
            balance: String::from("balance"),
        }
    }

    pub fn root(
        &self,
        access_context: &AccessContext,
    ) -> Option<http::request::Request<hyper::Body>> {
        let uri = format!("{}{}", access_context.base.base_uri, self.root);
        create_get_request_with_uri(&access_context.access_token, &uri)
    }

    pub fn balance(
        &self,
        access_context: &AccessContext,
    ) -> Option<http::request::Request<hyper::Body>> {
        let uri = format!(
            "{}{}/{}",
            access_context.base.base_uri, self.root, self.balance
        );
        create_get_request_with_uri(&access_context.access_token, &uri)
    }

    pub async fn extract_basic_info(body: hyper::Body) -> Option<models::BasicInfo> {
        read_body::<models::BasicInfo>(body)
            .await
    }

    pub async fn extract_balance(body: hyper::Body) -> Option<models::Balance> {
        read_body::<models::Balance>(body)
            .await
    }
}

fn create_default_request_builder(token: &str) -> http::request::Builder {
    http::request::Builder::new()
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &token))
}

fn create_get_request_with_uri(
    token: &str,
    uri: &str,
) -> Option<http::request::Request<hyper::Body>> {
    create_default_request_builder(token)
        .method(hyper::Method::GET)
        .uri(uri)
        .body(hyper::Body::empty())
        .ok()
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
