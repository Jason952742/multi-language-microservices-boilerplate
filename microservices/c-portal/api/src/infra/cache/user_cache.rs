use std::str::FromStr;
use uuid::Uuid;
use shared::datasource::dragonfly::DragonflyPool;
use shared::redis::{AsyncCommands, RedisError};
use futures::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use shared::utils::{to_datetime, to_uuid};
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::cache_user::CacheUser;

pub async fn get_user(user_id: Uuid) -> Result<CacheUser, RedisError> {
    let client = DragonflyPool::client(2).await;
    let mut con = client.get_async_connection().await?;
    let key = format!("user-{}", user_id.to_string());

    let user_id: String = con.hget(&key, "user_id").await?;
    let user_name: String = con.hget(&key, "user_name").await?;
    let member_id: String = con.hget(&key, "member_id").await?;
    let member_type: String = con.hget(&key, "member_type").await?;
    let member_status: String = con.hget(&key, "member_status").await?;
    let sub_end_date: String = con.hget(&key, "sub_end_date").await?;
    let account_id: String = con.hget(&key, "account_id").await?;
    let account_balance: f64 = con.hget(&key, "account_balance").await?;
    let referral_code: String = con.hget(&key, "referral_code").await?;
    let last_login_at: String = con.hget(&key, "last_login_at").await?;

    let res = CacheUser {
        user_id: to_uuid(&user_id),
        user_name,
        member_id: to_uuid(&member_id),
        member_type: MemberType::from_str(&member_type).unwrap(),
        member_status: MemberStatus::from_str(&member_status).unwrap(),
        sub_end_date: to_datetime(&sub_end_date),
        account_id: to_uuid(&account_id),
        account_balance: Decimal::try_from(account_balance).unwrap(),
        referral_code,
        last_login_at: to_datetime(&last_login_at),
    };

    Ok(res)

}

pub async fn set_user(user: CacheUser) -> Result<(), RedisError> {
    let client = DragonflyPool::client(2).await;
    let mut con = client.get_async_connection().await?;
    let key = format!("user-{}", &user.user_id.to_string());

    con.hset(&key, "user_id", user.user_id.to_string()).await?;
    con.hset(&key, "user_name", user.user_name).await?;
    con.hset(&key, "member_id", user.member_id.to_string()).await?;
    con.hset(&key, "member_type", user.member_type.to_string()).await?;
    con.hset(&key, "member_status", user.member_status.to_string()).await?;
    con.hset(&key, "sub_end_date", user.sub_end_date.to_string()).await?;
    con.hset(&key, "account_id", user.account_id.to_string()).await?;
    con.hset(&key, "account_balance", user.account_balance.to_f64()).await?;
    con.hset(&key, "referral_code", user.referral_code.to_string()).await?;
    con.hset(&key, "last_login_at", user.last_login_at.to_string()).await?;

    Ok(())
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();
    let user = CacheUser {
        user_id: id,
        ..Default::default()
    };

    set_user(user).await?;

    let u = get_user(id).await?;

    println!("{:?}", u);

    Ok(())
}