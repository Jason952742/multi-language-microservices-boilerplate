use sea_orm::entity::prelude::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MemberStatus {
    #[sea_orm(string_value = "Created")]
    #[default]
    Created,
    #[sea_orm(string_value = "Enabled")]
    Enabled, // If in use, cannot delete
    #[sea_orm(string_value = "Blocked")]
    Blocked,
    #[sea_orm(string_value = "Disabled")]
    Disabled,
    #[sea_orm(string_value = "Deleted")]
    Deleted, // Soft deletes
}

#[derive(Default, Debug, Clone, PartialEq, Eq, EnumString, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum MemberType {
    #[sea_orm(string_value = "Wood")]
    #[default]
    Wood,
    #[sea_orm(string_value = "Iron")]
    Iron,
    #[sea_orm(string_value = "Brass")]
    Brass,
    #[sea_orm(string_value = "Silver")]
    Silver,
    #[sea_orm(string_value = "Gold")]
    Gold,
    #[sea_orm(string_value = "Platinum")]
    Platinum,
    #[sea_orm(string_value = "Diamond")]
    Diamond,
    #[sea_orm(string_value = "Sphene")]
    Sphene,
}
