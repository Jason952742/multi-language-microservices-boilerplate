use std::str::FromStr;
use axum::{Json, Router};
use axum::routing::{delete, post};
use axum::http::StatusCode;
use chrono::{Utc};
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use tracing::debug;
use uuid::Uuid;
use validator::Validate;
use shared::bson::doc;
use shared::datasource::mongo::MongoPool;
use shared::utils::{AuthError, to_uuid};
use shared::utils::{CustomError, CustomResponseBuilder, ValidatedForm, ValidatedJson};
use shared::utils::CustomResponseResult as Response;
use crate::infra::repositories::{SettingsDbMutation, SettingsDbQuery};
use crate::application::restful::keycloak_client;
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::cache_user::CacheUser;
use crate::infra::dto::user::{AuthenticateResponse, AuthorizeBody, CreateBody};
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn sessions_routes() -> Router<> {
    Router::new()
        .route("/sessions", post(authenticate_user))
        .route("/sessions", delete(unauthenticate_user))
}

#[derive(Debug, Default, Clone, Deserialize, Validate)]
struct CheckParm {
    username: String,
}


async fn authenticate_user(ValidatedJson(body): ValidatedJson<AuthorizeBody>) -> Result<Json<AuthenticateResponse>, CustomError> {
    let admin_token = keycloak_client::get_admin_token().await?;
    match keycloak_client::get_user_token(&body.identifier, &body.password).await {
        Ok(token) => {
            let claim = keycloak_client::get_user_by_token(&token.access_token).await?;
            // TODO: get member / account / referral info
            // let member = ???
            let cached_user = CacheUser {
                user_id: to_uuid(&claim.sub),
                user_name: claim.preferred_username,
                member_id: Uuid::default(),
                member_type: MemberType::Wood,
                member_status: MemberStatus::Created,
                sub_end_date: Utc::now(),
                account_id: Uuid::default(),
                account_balance: dec!(0),
                referral_code: "".to_string(),
                last_login_at: Utc::now(),
            };

            let res = AuthenticateResponse {
                user: cached_user,
                access_token: token.access_token,
            };
            Ok(Json(res))
        }
        Err(e) => {
            Err(CustomError::Authenticate(AuthError::WrongCredentials))
        }
    }
}

async fn unauthenticate_user(form: ValidatedForm<UserSettingsForm>) -> Response<UserSettingsItem> {
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
