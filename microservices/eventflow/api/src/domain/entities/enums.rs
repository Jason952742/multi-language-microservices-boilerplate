use std::str::FromStr;
use scylla::_macro_internal::{CellWriter, ColumnType, CqlValue, FromCqlVal, FromCqlValError, SerializationError, SerializeCql, WrittenCellProof};
use serde_derive::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString, Display};

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, Deserialize, Serialize)]
pub enum MemberStatus {
    #[default]
    Created,
    Enabled, // If in use, cannot delete
    Blocked,
    Disabled,
    Deleted, // Soft deletes
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Deserialize, Serialize, Display)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Deserialize, Serialize)]
pub enum TransferType {
    #[default]
    In,
    Out,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Deserialize, Serialize, Display)]
pub enum TransactionStatus {
    #[default]
    Apply,
    Pending,
    Completed,
    Failure,
    Handled,
    Canceled,
    Ignored,
}

impl FromCqlVal<CqlValue> for TransactionStatus {
    fn from_cql(cql_val: CqlValue) -> anyhow::Result<Self, FromCqlValError> {
        let str = cql_val.into_string().ok_or(FromCqlValError::BadCqlType)?;
        TransactionStatus::from_str(&str).map_err(|_| FromCqlValError::BadVal)
    }
}

impl SerializeCql for TransactionStatus {
    fn serialize<'b>(&self, _typ: &ColumnType, writer: CellWriter<'b>) -> Result<WrittenCellProof<'b>, SerializationError> {
        let value = self.to_string();
        writer.set_value(value.as_ref()).map_err(|e| SerializationError::new(e))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Deserialize, Serialize, Display)]
pub enum TransactionType {
    #[default]
    None,
    UserCreate,
    AccountDeposit,
    AccountWithdraw,
    MemberSubscription,
}

impl FromCqlVal<CqlValue> for TransactionType {
    fn from_cql(cql_val: CqlValue) -> anyhow::Result<Self, FromCqlValError> {
        let str = cql_val.into_string().ok_or(FromCqlValError::BadCqlType)?;
        TransactionType::from_str(&str).map_err(|_| FromCqlValError::BadVal)
    }
}

impl SerializeCql for TransactionType {
    fn serialize<'b>(&self, _typ: &ColumnType, writer: CellWriter<'b>) -> Result<WrittenCellProof<'b>, SerializationError> {
        let value = self.to_string();
        writer.set_value(value.as_ref()).map_err(|e| SerializationError::new(e))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, Deserialize, Serialize, Display)]
pub enum AggregateType {
    #[default]
    Member,
    Account,
    Referral,
}

impl FromCqlVal<CqlValue> for AggregateType {
    fn from_cql(cql_val: CqlValue) -> anyhow::Result<Self, FromCqlValError> {
        let str = cql_val.into_string().ok_or(FromCqlValError::BadCqlType)?;
        AggregateType::from_str(&str).map_err(|_| FromCqlValError::BadVal)
    }
}

impl SerializeCql for AggregateType {
    fn serialize<'b>(&self, _typ: &ColumnType, writer: CellWriter<'b>) -> Result<WrittenCellProof<'b>, SerializationError> {
        let value = self.to_string();
        writer.set_value(value.as_ref()).map_err(|e| SerializationError::new(e))
    }
}