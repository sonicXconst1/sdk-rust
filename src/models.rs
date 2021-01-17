use super::coin;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_at: i64,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct BasicInfo {
    pub id: i64,
    pub merchant_info: Option<MerchantInfo>,
    pub profile: Profile,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct MerchantInfo {
    pub name: String,
    pub usd_amount_max_limit: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
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

#[derive(serde::Deserialize, Clone, Debug)]
pub struct AML5Limits {
    pub current_turnover: String,
    pub current_withdraw: String,
    pub turnover_limit: String,
    pub withdraw_limit: String,
    pub withdraw_limit_daily: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Verification {
    pub current_level: String,
}

pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Currency {
    pub amount: String,
    pub coin: String,
    pub held: String,
}

pub type Coins = Vec<Coin>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Coin {
    pub decimals: u32,
    pub full_name: String,
    pub name: String,
}

pub type Orders = Vec<Order>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Order {
    pub amount: String,
    pub created_at: String,
    pub id: u32,
    pub initial_amount: Option<String>,
    pub is_owner: Option<bool>,
    pub pair: String,
    pub rate: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct OrderRequest {
    pub amount: String,
    pub pair: String,
    pub rate: String,
}

impl OrderRequest {
    pub fn new(pair: coin::CoinPair, amount: f64, rate: f64) -> OrderRequest {
        OrderRequest {
            pair: pair.into(),
            amount: amount.to_string(),
            rate: rate.to_string(),
        }
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct UpdateOrder {
    pub amount: String,
    pub rate: String,
}

pub type Trades = Vec<Trade>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Trade {
    amount: String,
    created_at: String,
    fee: String,
    id: u32,
    order: Order,
    received_amount: String,
    updated_at: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateTradeRequest {
    pub amount: String,
    pub rate: String,
}

pub type Invoices = Vec<Invoice>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Invoice {
    pub amount: f64,
    pub callback_url: String,
    pub coin: String,
    pub country_code: String,
    pub created_at: String,
    pub fiat: String,
    pub id: String,
    pub lang_id: String,
    pub payment_system_id: PaymentSystemId,
    pub payment_url: String,
    pub redirect_url: String,
    pub status: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateInvoice {
    pub amount: String,
    pub callback_url: String,
    pub coin: String,
    pub country_code: String,
    pub data: String,
    pub fiat: String,
    pub lang_id: String,
    pub payment_system_id: String,
    pub redirect_url: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub enum InvoiceStatus {
    Unassigned,
    Active,
    Completed,
    Canceled,
}

pub type PaymentSystemId = u32;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PaymentSystem {
    id: PaymentSystemId,
    name: String,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Estimate {
    amount: String,
    coin: String,
}

pub type FiatEstimations = Vec<FiatEstimation>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct FiatEstimation {
    pub estimations: PaymentSystemEstimations,
    pub fiat: Fiat,
}

pub type PaymentSystemEstimations = Vec<PaymentSystemEstimation>;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct PaymentSystemEstimation {
    pub estimated_fiat_amount: f64,
    pub payment_system: PaymentSystem,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Fiat {
    pub decimals: u32,
    pub full_name: String,
    pub name: String,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            InvoiceStatus::Unassigned => formatter.write_str("UNASSIGNED"),
            InvoiceStatus::Active => formatter.write_str("ACTIVE"),
            InvoiceStatus::Completed => formatter.write_str("COMPLETED"),
            InvoiceStatus::Canceled => formatter.write_str("CANCELED"),
        }
    }
}
