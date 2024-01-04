use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, Deserialize, Serialize)]
pub enum MemberStatus {
    #[default]
    Created,
    Enabled, // If in use, cannot delete
    Blocked,
    Disabled,
    Deleted, // Soft deletes
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, Deserialize, Serialize)]
pub enum MemberType {
    #[default]
    Wood,
    Iron,
    Brass,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Sphene,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, Deserialize, Serialize)]
pub enum PaymentType {
    #[default]
    WechatPay,
    Alipay,
    BankCash,
    BitcoinLightingNetwork,
    BscNetwork,
    BtcNetwork,
    AccountToken,
    AccountBalance,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, Deserialize, Serialize)]
pub enum CurrencyType {
    #[default]
    USD,
    EUR,
    CNY,
    HKD,
    SGD,
    USDT,
    USDC,
    BTC,
    ETH,
    SOL,
    ATOM,
    DOGE,
    BNB,
    GPT,
    XRP,
    FIL,
    DOT,
}
