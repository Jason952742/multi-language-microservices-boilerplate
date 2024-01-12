use std::str::FromStr;
use axum::{Json, Router};
use axum::routing::{post, put};
use serde_derive::Deserialize;
use validator::Validate;
use shared::bson::doc;
use shared::utils::{CustomError, CustomResponse, ValidatedJson, ValidatedPath};
use shared::utils::CustomResponseResult as Response;
use shared::utils::requests::pagination::PaginationQuery;
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn credential_routes() -> Router<> {
    Router::new()
        .route("/credential/:id/password", put(change_password))
        .route("/credential/:id/forgot-password", post(forgot_password))
        .route("/credential/reset-password/:token", put(reset_password))
}

#[derive(Debug, Default, Clone, Deserialize, Validate)]
struct CheckParm {
    username: String,
}

async fn change_password(ValidatedPath(id): ValidatedPath<String>) -> Result<CustomResponse<()>, CustomError> {
    todo!()
}

async fn forgot_password(ValidatedPath(id): ValidatedPath<String>, ValidatedJson(payload): ValidatedJson<UserSettingsForm>) -> Result<Json<UserSettingsItem>, CustomError> {
    todo!()
}

async fn reset_password(pagination: PaginationQuery) -> Response<Vec<UserSettingsItem>> {
    todo!()
}

async fn bind_email(ValidatedPath(id): ValidatedPath<String>) -> Result<Json<UserSettingsItem>, CustomError> {
    todo!()
}
