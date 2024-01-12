use std::str::FromStr;
use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{get, post, put};
use axum::http::StatusCode;
use chrono::{Utc};
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use validator::Validate;
use shared::bson::doc;
use shared::utils::{parse_code, to_datetime, to_uuid};
use shared::utils::{CustomError, CustomResponse, CustomResponseBuilder, ValidatedJson, ValidatedPath};
use crate::application::grpc::eventflow_client;
use crate::application::restful::keycloak_client;
use crate::application::services::{referral_refresh_svc, token_refresh_svc};
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::cache_user::CacheUser;
use crate::infra::cache::{referral_cache, user_cache};
use crate::infra::dto::user::{AuthenticateResponse, CreateBody};
use crate::infra::dto::user_settings::{UserSettingsItem};

pub fn user_routes() -> Router<> {
    Router::new()
        .route("/users", get(check_user))
        .route("/users", post(create_user))
        .route("/users/{id}/email", put(bind_email))
}

#[derive(Debug, Default, Clone, Deserialize, Validate)]
struct CheckParm {
    username: String,
}

async fn check_user(Query(parm): Query<CheckParm>) -> Result<String, CustomError> {
    let username = parm.username;
    let token = keycloak_client::get_admin_token().await?;
    let user = keycloak_client::get_user_by_name(&username, &token.access_token).await?;

    Ok(user.is_some().to_string())
}

async fn create_user(ValidatedJson(body): ValidatedJson<CreateBody>) -> Result<CustomResponse<AuthenticateResponse>, CustomError> {
    let admin_token = keycloak_client::get_admin_token().await?;
    match keycloak_client::get_user_by_name(&body.identifier, &admin_token.access_token).await? {
        None => {
            // check referrer
            let referrer_id = if body.referral_code.is_some() {
                referral_refresh_svc::get_or_refresh(&body.referral_code.clone().unwrap()).await?
            } else { None };
            // keycloak create user
            let id = keycloak_client::create_user(&body.identifier, &body.password, &admin_token.access_token).await?.unwrap();
            let user_id = to_uuid(&id);

            // event flow
            let created_user = eventflow_client::user_create(user_id.clone(), &body.identifier, referrer_id, body.referral_code).await?;

            // event flow success
            if created_user.code == parse_code(tonic::Code::Ok) {
                let user = created_user.data;
                let cached_user = CacheUser {
                    user_id: user_id.clone(),
                    user_name: user.user_name,
                    member_id: to_uuid(&user.member_id),
                    member_type: MemberType::from_str(&user.member_type).unwrap(),
                    member_status: MemberStatus::Created,
                    sub_end_date: to_datetime(&user.subscription_end_date),
                    account_id: to_uuid(&user.account_id),
                    account_balance: dec!(0),
                    referral_code: user.refer_code.clone(),
                    last_login_at: Utc::now(),
                };
                let user_token = keycloak_client::get_user_token(&body.identifier, &body.password).await?;
                // cache referral code
                let _ = referral_cache::set_referral(&user.refer_code, user_id.clone()).await?;
                // cache user info
                let _ = user_cache::set_user(cached_user.clone()).await?;
                let _ = token_refresh_svc::remove_and_refresh(user_id.clone(), user_token.clone()).await?;

                let res = CustomResponseBuilder::new()
                    .body(AuthenticateResponse {
                        access_token: user_token.access_token,
                        user: cached_user,
                    })
                    .status_code(StatusCode::CREATED)
                    .build();

                Ok(res)
            } else {
                // todo: rollback keycloak, If the event write fails, request to keycloak to remove the user
                Err(CustomError::not_found())
            }
        }
        Some(_) => Err(CustomError::already_exists())
    }
}

async fn bind_email(ValidatedPath(id): ValidatedPath<String>) -> Result<Json<UserSettingsItem>, CustomError> {
    todo!()
}
