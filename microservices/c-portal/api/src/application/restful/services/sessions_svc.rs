use std::ops::Add;
use std::str::FromStr;
use axum::{Json, Router};
use axum::routing::{delete, post};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use tracing::debug;
use uuid::Uuid;
use validator::Validate;
use shared::bson::doc;
use shared::datasource::mongo::MongoPool;
use shared::redis::RedisError;
use shared::utils::{AuthError, to_datetime, to_uuid};
use shared::utils::{CustomError, CustomResponseBuilder, ValidatedForm, ValidatedJson};
use shared::utils::CustomResponseResult as Response;
use crate::application::grpc::{member_client, referral_client};
use crate::infra::repositories::{SettingsDbMutation, SettingsDbQuery};
use crate::application::restful::keycloak_client;
use crate::domain::entities::cache_token::{CacheRefreshToken, CacheToken};
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::cache_user::CacheUser;
use crate::infra::cache::{referral_cache, refresh_cache, token_cache, user_cache};
use crate::infra::dto::user::{AuthenticateResponse, AuthorizeBody};
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn sessions_routes() -> Router<> {
    Router::new()
        .route("/sessions", post(authenticate))
        .route("/sessions", delete(unauthenticate))
}

#[derive(Debug, Default, Clone, Deserialize, Validate)]
struct CheckParm {
    username: String,
}

async fn authenticate(ValidatedJson(body): ValidatedJson<AuthorizeBody>) -> Result<Json<AuthenticateResponse>, CustomError> {
    match keycloak_client::get_user_token(&body.identifier, &body.password).await {
        Ok(user_token) => {
            let claim = keycloak_client::get_user_by_token(&user_token.access_token).await?;
            let user_id = to_uuid(&claim.sub);

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

                    // cache referral code
                    let _ = referral_cache::set_referral(&referral.referral_code, user_id.clone()).await?;
                    // cache user info
                    let _ = user_cache::set_user(cached_user.clone()).await?;

                    Ok(cached_user)
                }
            };

            // cache access token
            let _ = token_cache::set_token(
                &user_token.access_token,
                CacheToken {
                    user_id: user_id.clone(),
                    expires_date: Utc::now().add(Duration::seconds(user_token.expires_in)),
                },
                user_token.refresh_expires_in,
            ).await?;
            // cache refresh token
            let _ = refresh_cache::set_refresh_token(
                user_id,
                CacheRefreshToken {
                    access_token: user_token.access_token.clone(),
                    refresh_token: user_token.refresh_token,
                },
                user_token.refresh_expires_in,
            ).await?;


            let res = AuthenticateResponse { user: user?, access_token: user_token.access_token };
            Ok(Json(res))
        }
        Err(_) => Err(CustomError::Authenticate(AuthError::WrongCredentials))
    }
}

async fn unauthenticate(form: ValidatedForm<UserSettingsForm>) -> Response<UserSettingsItem> {
    let form = form.0;
    let model = form.into();
    let conn = MongoPool::conn().await;

    let oid = SettingsDbMutation::create_settings(conn, model)
        .await.map_err(|e| CustomError::Mongo(e))?;

    match SettingsDbQuery::find_settings_by_id(conn, oid)
        .await.map_err(|e| CustomError::Mongo(e))? {
        Some(x) => {
            let res = CustomResponseBuilder::new()
                .body(UserSettingsItem::from(x))
                .status_code(StatusCode::CREATED)
                .build();
            Ok(res)
        }
        None => {
            debug!("Cat not found, returning 404 status code");
            return Err(CustomError::not_found());
        }
    }
}
