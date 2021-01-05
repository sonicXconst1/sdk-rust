#[derive(serde::Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: i32
}

#[derive(serde::Deserialize, Debug)]
pub struct BasicInfo {
    pub id: i64,
    pub merchant_info: Option<MerchantInfo>,
    pub profile: Profile
}

#[derive(serde::Deserialize, Debug)]
pub struct MerchantInfo {
    pub name: String,
    pub usd_amount_max_limit: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Profile {
    pub country_code: String,
    pub email: Option<String>,
    pub is_finance_blocked: bool,
    pub lang_id: String,
    pub limits: AML5Limits,
    pub phone: String,
    pub username: String,
    pub verification: Verification,
}

#[derive(serde::Deserialize, Debug)]
pub struct AML5Limits {
    current_turnover: String,
    current_withdraw: String,
    turnover_limit: String,
    withdraw_limit: String,
    withdraw_limit_daily: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Verification {
    current_level: String
}

pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Debug)]
pub struct Currency {
    amount: String,
    coin: String,
    held: String,
}

pub type Coins = Vec<Coin>;

#[derive(serde::Deserialize, Debug)]
pub struct Coin {
    decimals: u32,
    full_name: String,
    name: String
}
