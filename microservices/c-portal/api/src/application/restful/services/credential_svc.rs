use axum::{Json, Router};
use axum::routing::{post, put};
use serde_derive::Deserialize;
use validator::Validate;
use shared::bson::doc;
use shared::utils::{CustomError, CustomResponse, to_uuid, ValidatedJson, ValidatedPath};
use shared::utils::CustomResponseResult as Response;
use shared::utils::requests::pagination::PaginationQuery;
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn credential_routes() -> Router<> {
    Router::new()
        .route("/credential/:id/password", put(change_password))
        .route("/credential/:id/forgot-password", post(forgot_password))
        .route("/credential/reset-password/:token", put(reset_password))
}

async fn change_password(ValidatedPath(id): ValidatedPath<String>, ValidatedJson(form): ValidatedJson<PasswordBody>) -> Result<CustomResponse<()>, CustomError> {
    let user_id = to_uuid(&id);




    todo!()
}

async fn forgot_password() -> Result<(), CustomError> {
    todo!()
}

async fn reset_password() -> Result<(), CustomError> {
    todo!()
}
