use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

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
