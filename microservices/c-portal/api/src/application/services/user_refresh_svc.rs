use std::str::FromStr;
use chrono::Utc;
use rust_decimal_macros::dec;
use uuid::Uuid;
use shared::keycloak_api::UserClaim;
use shared::redis::RedisError;
use shared::utils::{CustomError, to_datetime, to_uuid};
use crate::application::grpc::{member_client, referral_client};
use crate::domain::entities::cache_user::CacheUser;
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::infra::cache::{user_cache};

pub async fn get_or_refresh(user_id: Uuid, claim: UserClaim) -> Result<CacheUser, CustomError> {
    let user: Result<CacheUser, RedisError> = match user_cache::get_user(user_id.clone()).await {
        Ok(u) => Ok(u),
        Err(_) => {
            let member = member_client::get_member(user_id.clone()).await?.data.unwrap();
            let referral = referral_client::get_referral_by_id(user_id.clone()).await?.data.unwrap();

            let cached_user = CacheUser {
                user_id: to_uuid(&claim.sub),
                user_name: claim.preferred_username,
                member_id: to_uuid(&member.id),
                member_type: MemberType::from_str(&member.member_type).unwrap(),
                member_status: MemberStatus::from_str(&member.status).unwrap(),
                sub_end_date: to_datetime(&member.sub_end_date),
                account_id: Uuid::default(),
                account_balance: dec!(0),
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