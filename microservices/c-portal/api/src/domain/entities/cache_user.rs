use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::entities::enums::{MemberStatus, MemberType};


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheUser {
  pub user_id: Uuid,
  pub user_name: String,
  pub member_id: Uuid,
  pub member_type: MemberType,
  pub member_status: MemberStatus,
  pub sub_end_date: DateTime<Utc>,
  pub account_id: Uuid,
  pub account_balance: Decimal,
  pub referral_code: String,
  pub last_login_at: DateTime<Utc>,
}
