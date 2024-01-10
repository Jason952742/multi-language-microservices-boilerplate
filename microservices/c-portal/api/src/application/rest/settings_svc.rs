use axum::{Json, Router};
use axum::routing::{get, delete, post, put};
use axum::http::StatusCode;
use tracing::debug;
use shared::bson::doc;
use shared::datasource::mongo::MongoPool;
use shared::utils::to_object_id;
use shared::utils::{CustomError, CustomResponse, CustomResponseBuilder, ResponsePagination, ValidatedForm, ValidatedJson, ValidatedPath};
use shared::utils::CustomResponseResult as Response;
use crate::infra::repositories::{SettingsDbMutation, SettingsDbQuery};
use crate::domain::entities::{user_settings};
use shared::utils::requests::pagination::PaginationQuery;
use crate::infra::dto::user_settings::{UserSettingsForm, UserSettingsItem};

pub fn settings_routes() -> Router<> {
    Router::new()
        .route("/settings", post(create_settings))
        .route("/settings", get(query_settings))
        .route("/settings/:id", get(get_settings_by_id))
        .route("/settings/:id", delete(remove_settings_by_id))
        .route("/settings/:id", put(update_settings_by_id))
}

async fn create_settings(form: ValidatedForm<UserSettingsForm>) -> Response<UserSettingsItem> {
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

async fn remove_settings_by_id(ValidatedPath(id): ValidatedPath<String>) -> Result<CustomResponse<()>, CustomError> {
    let oid = to_object_id(id.clone()).map_err(|_| CustomError::ParseObjectID(id))?;
    let conn = MongoPool::conn().await;

    SettingsDbMutation::delete_settings(conn, oid)
        .await.map_err(|e| CustomError::Mongo(e))?;

    let res = CustomResponseBuilder::new()
        .status_code(StatusCode::NO_CONTENT)
        .build();

    Ok(res)
}

async fn update_settings_by_id(ValidatedPath(id): ValidatedPath<String>, ValidatedJson(payload): ValidatedJson<UserSettingsForm>) -> Result<Json<UserSettingsItem>, CustomError> {
    let oid = to_object_id(id.clone()).map_err(|_| CustomError::ParseObjectID(id))?;
    let conn = MongoPool::conn().await;

    match SettingsDbQuery::find_settings_by_id(conn, oid)
        .await.map_err(|e| CustomError::Mongo(e))? {
        Some(x) => {
            let model = user_settings::Model {
                user_id: x.user_id,
                theme: payload.theme,
                language: payload.language,
                ..x
            };
            SettingsDbMutation::update_settings_by_id(conn, oid, model.clone())
                .await.map_err(|e| CustomError::Mongo(e))?;

            Ok(Json(UserSettingsItem::from(model)))
        }
        None => {
            debug!("Cat not found, returning 404 status code");
            return Err(CustomError::not_found());
        }
    }
}

async fn query_settings(pagination: PaginationQuery) -> Response<Vec<UserSettingsItem>> {
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

async fn get_settings_by_id(ValidatedPath(id): ValidatedPath<String>) -> Result<Json<UserSettingsItem>, CustomError> {
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
