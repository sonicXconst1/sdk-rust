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
    pub current_turnover: String,
    pub current_withdraw: String,
    pub turnover_limit: String,
    pub withdraw_limit: String,
    pub withdraw_limit_daily: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Verification {
    pub current_level: String
}

pub type Balance = Vec<Currency>;

#[derive(serde::Deserialize, Debug)]
pub struct Currency {
    pub amount: String,
    pub coin: String,
    pub held: String,
}

pub type Coins = Vec<Coin>;

#[derive(serde::Deserialize, Debug)]
pub struct Coin {
    pub decimals: u32,
    pub full_name: String,
    pub name: String
}

pub type Orders = Vec<Order>;

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Serialize, Debug)]
pub struct OrderRequest {
    pub amount: String,
    pub pair: String,
    pub rate: String,
}

#[derive(serde::Serialize, Debug)]
pub struct UpdateOrder {
    pub amount: String,
    pub rate: String,
}

pub type Trades = Vec<Trade>;

#[derive(serde::Deserialize, Debug)]
pub struct Trade {
    amount: String,
    created_at: String,
    fee: String,
    id: u32,
    order: Order,
    received_amount: String,
    updated_at: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateTradeRequest {
    pub amount: String,
    pub rate: String,
}

pub type Invoices = Vec<Invoice>;

#[derive(serde::Deserialize, Debug)]
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

pub type PaymentSystemId = u32;

#[derive(serde::Serialize, Debug)]
pub enum InvoiceStatus {
    Unassigned,
    Active,
    Completed,
    Canceled,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> Result<(), std::fmt::Error> {
        match self {
            InvoiceStatus::Unassigned => formatter.write_str("UNASSIGNED"),
            InvoiceStatus::Active => formatter.write_str("ACTIVE"),
            InvoiceStatus::Completed => formatter.write_str("COMPLETED"),
            InvoiceStatus::Canceled => formatter.write_str("CANCELED")
        }
    }
}
