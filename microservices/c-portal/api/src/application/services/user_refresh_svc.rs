use crate::application::grpc::{account_client, member_client, referral_client};
use crate::domain::entities::cache_user::CacheUser;
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::infra::cache::user_cache;
use chrono::Utc;
use rust_decimal::Decimal;
use shared::keycloak_api::UserClaim;
use shared::redis::RedisError;
use shared::utils::{to_datetime, to_uuid, CustomError};
use std::str::FromStr;
use uuid::Uuid;

pub async fn get_or_refresh(user_id: Uuid, claim: UserClaim) -> Result<CacheUser, CustomError> {
  let user: Result<CacheUser, RedisError> = match user_cache::get_user(user_id.clone()).await {
    Ok(u) => Ok(u),
    Err(_) => {
      let member = member_client::get_member(user_id.clone()).await?.data.unwrap();
      let referral = referral_client::get_referral_by_id(user_id.clone()).await?.data.unwrap();
      let accounts = account_client::get_account(user_id.clone()).await?.data;
      let account = accounts.first()?;

      let cached_user = CacheUser {
        user_id: to_uuid(&claim.sub),
        user_name: claim.preferred_username,
        member_id: to_uuid(&member.id),
        member_type: MemberType::from_str(&member.member_type).unwrap(),
        member_status: MemberStatus::from_str(&member.status).unwrap(),
        sub_end_date: to_datetime(&member.sub_end_date),
        account_id: to_uuid(&account.account_id),
        account_balance: Decimal::try_from(account.balance).unwrap(),
        referral_code: referral.referral_code.clone(),
        last_login_at: Utc::now(),
      };
      // cache user info
      let _ = user_cache::set_user(cached_user.clone()).await?;
      Ok(cached_user)
    }
  };
  Ok(user?)
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  Ok(())
}
