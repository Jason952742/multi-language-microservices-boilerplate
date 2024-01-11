use std::ops::Add;
use std::str::FromStr;
use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{get, delete, post, put};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;
use serde_derive::Deserialize;
use tracing::debug;
use validator::Validate;
use shared::bson::doc;
use shared::datasource::mongo::MongoPool;
use shared::utils::{parse_code, to_datetime, to_object_id, to_uuid};
use shared::utils::{CustomError, CustomResponse, CustomResponseBuilder, ResponsePagination, ValidatedForm, ValidatedJson, ValidatedPath};
use shared::utils::CustomResponseResult as Response;
use crate::infra::repositories::{SettingsDbMutation, SettingsDbQuery};
use shared::utils::requests::pagination::PaginationQuery;
use crate::application::grpc::eventflow_client;
use crate::application::restful::keycloak_client;
use crate::application::services::referral_svc;
use crate::domain::entities::cache_token::{CacheRefreshToken, CacheToken};
use crate::domain::entities::enums::{MemberStatus, MemberType};
use crate::domain::entities::cache_user::CacheUser;
use crate::infra::cache::{referral_cache, refresh_cache, token_cache, user_cache};
use crate::infra::dto::user::{AuthenticateResponse, AuthorizeBody, CreateBody};
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn auth_routes() -> Router<> {
    Router::new()
        .route("/users", get(check_user))
        .route("/users", post(create_user))
        .route("/sessions", post(authenticate_user))
        .route("/sessions", put(refrresh_user))
        .route("/sessions", delete(unauthenticate_user))
        .route("/users/:id/password", put(change_password))
        // .route("/users/:id/forgot-password", post(forgot_password))
        .route("/reset-password/:token", put(reset_password))
        .route("/users/{id}/email", put(bind_email))
}

#[derive(Debug, Default, Clone, Deserialize, Validate)]
struct CheckParm {
    username: String,
}

async fn check_user(Query(parm): Query<CheckParm>) -> Result<String, CustomError> {
    let username = parm.username;
    let token = keycloak_client::get_admin_token().await?;
    let user = keycloak_client::get_user(&username, &token.access_token).await?;

    Ok(user.is_some().to_string())
}

async fn create_user(ValidatedJson(body): ValidatedJson<CreateBody>) -> Result<CustomResponse<AuthenticateResponse>, CustomError> {
    let admin_token = keycloak_client::get_admin_token().await?;
    match keycloak_client::get_user(&body.identifier, &admin_token.access_token).await? {
        None => {
            // check referrer
            let referrer_id = if (body.referral_code.is_some()) {
                referral_svc::get_referral(&body.referral_code.clone().unwrap()).await?
            } else { None };
            // keycloak create user
            let id = keycloak_client::create_user(&body.identifier, &body.password, &admin_token.access_token).await?.unwrap();
            let user_id = to_uuid(&id);

            // event flow
            let created_user = eventflow_client::user_create(user_id.clone(), &body.identifier, referrer_id, body.referral_code).await?;

            // event flow success
            if (created_user.code == parse_code(tonic::Code::Ok)) {
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
                // cache access token
                let _ = token_cache::set_token(
                    &user_token.access_token,
                    CacheToken {
                        user_id: user_id.clone(),
                        expires_date: Utc::now().add(Duration::seconds(user_token.expires_in)),
                    },
                    user_token.refresh_expires_in
                );
                // cache refresh token
                let _ = refresh_cache::set_refresh_token(
                    user_id,
                    CacheRefreshToken {
                        access_token: user_token.access_token.clone(),
                        refresh_token: user_token.refresh_token,
                    },
                    user_token.refresh_expires_in
                );

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

async fn authenticate_user(
    Json(body): Json<AuthorizeBody>,
) -> Result<Json<AuthenticateResponse>, CustomError> {
    let email = &body.identifier;
    let password = &body.password;

    // let user = User::find_one(doc! { "email": email }, None).await?;

    // if user.locked_at.is_some() {
    //     debug!("User is locked, returning 401");
    //     return Err(CustomError::Authenticate(AuthError::Locked));
    // }

    // let secret = SETTINGS.auth.secret.as_str();
    // let token = token::create(user.clone(), secret)
    //     .map_err(|_| Error::Authenticate(AuthError::TokenCreation))?;
    //
    // let res = AuthenticateResponse {
    //     access_token: token,
    //     user: PublicUser::from(user),
    // };

    // Ok(Json(res))
    todo!()
}

async fn refrresh_user() -> Result<(), CustomError> {
    Ok(())
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

async fn change_password(ValidatedPath(id): ValidatedPath<String>) -> Result<CustomResponse<()>, CustomError> {
    let oid = to_object_id(id.clone()).map_err(|_| CustomError::ParseObjectID(id))?;
    let conn = MongoPool::conn().await;

    SettingsDbMutation::delete_settings(conn, oid)
        .await.map_err(|e| CustomError::Mongo(e))?;

    let res = CustomResponseBuilder::new()
        .status_code(StatusCode::NO_CONTENT)
        .build();

    Ok(res)
}

// async fn forgot_password(ValidatedPath(id): ValidatedPath<String>, ValidatedJson(payload): ValidatedJson<UserSettingsForm>) -> Result<Json<UserSettingsItem>, CustomError> {
//     let oid = to_object_id(id.clone()).map_err(|_| CustomError::ParseObjectID(id))?;
//     let conn = MongoPool::conn().await;
//
//     match SettingsDbQuery::find_settings_by_id(conn, oid)
//         .await.map_err(|e| CustomError::Mongo(e))? {
//         Some(x) => {
//             let model = user_settings::Model {
//                 user_id: x.user_id,
//                 theme: payload.theme,
//                 language: payload.language,
//                 ..x
//             };
//             SettingsDbMutation::update_settings_by_id(conn, oid, model.clone())
//                 .await.map_err(|e| CustomError::Mongo(e))?;
//
//             Ok(Json(UserSettingsItem::from(model)))
//         }
//         None => {
//             debug!("Cat not found, returning 404 status code");
//             return Err(CustomError::not_found());
//         }
//     }
// }

async fn reset_password(pagination: PaginationQuery) -> Response<Vec<UserSettingsItem>> {
    let conn = MongoPool::conn().await;

    let filter = doc! { };
    let find_options = None;
    let (num_pages, models) = SettingsDbQuery::find_settings_in_page(conn, filter, find_options, pagination.page, pagination.limit)
        .await.map_err(|e| CustomError::Mongo(e))?;

    let res = CustomResponseBuilder::new()
        .body(models.into_iter().map(|x| UserSettingsItem::from(x)).collect())
        .pagination(ResponsePagination {
            count: num_pages,
            offset: pagination.offset,
            limit: pagination.limit,
        })
        .build();

    debug!("Returning cats");
    Ok(res)
}

async fn bind_email(ValidatedPath(id): ValidatedPath<String>) -> Result<Json<UserSettingsItem>, CustomError> {
    let oid = to_object_id(id.clone()).map_err(|_| CustomError::ParseObjectID(id))?;
    let conn = MongoPool::conn().await;

    let opt = SettingsDbQuery::find_settings_by_id(conn, oid)
        .await.map_err(|e| CustomError::Mongo(e))?;

    match opt {
        Some(x) => {
            debug!("Returning settings");
            Ok(Json(UserSettingsItem::from(x)))
        }
        None => {
            debug!("Cat not found, returning 404 status code");
            return Err(CustomError::not_found());
        }
    }
}
