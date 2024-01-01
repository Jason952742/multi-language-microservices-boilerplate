use axum::extract::{Path, Query, State};
use axum::{Form, Router};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use tower_cookies::Cookies;
use shared::mongodb::bson::doc;
use shared::to_object_id;

use crate::domain::entities::post;
use crate::flash::{get_flash_cookie, post_response, PostResponse};
use crate::infra::{AppState, FlashData, Params, route};
use crate::infra::repositories::{PostOrmMutation, PostOrmQuery};

pub fn post_routes() -> Router<AppState> {
    route(
        "/",
        get(PostService::list_posts).post(PostService::create_post),
    ).route(
        "/:id",
        get(PostService::edit_post).post(PostService::update_post),
    ).route(
        "/new",
        get(PostService::new_post),
    ).route(
        "/delete/:id",
        post(PostService::delete_post),
    )
}

pub struct PostService;

impl PostService {
    pub async fn list_posts(state: State<AppState>, Query(params): Query<Params>, cookies: Cookies) -> Result<Html<String>, (StatusCode, &'static str)> {
        let page = params.page.unwrap_or(1);
        let posts_per_page = params.posts_per_page.unwrap_or(5);

        let filter = doc! { };
        let find_options = None;
        let (num_pages, posts) = PostOrmQuery::find_posts_in_page(&state.conn, filter, find_options, page, posts_per_page)
            .await
            .expect("Cannot find posts in page");

        let mut ctx = tera::Context::new();
        ctx.insert("posts", &posts);
        ctx.insert("page", &page);
        ctx.insert("posts_per_page", &posts_per_page);
        ctx.insert("num_pages", &num_pages);

        if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
            ctx.insert("flash", &value);
        }

        let body = state
            .templates
            .render("index.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn new_post(state: State<AppState>) -> Result<Html<String>, (StatusCode, &'static str)> {
        let ctx = tera::Context::new();
        let body = state
            .templates
            .render("new.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn create_post(state: State<AppState>, mut cookies: Cookies, form: Form<post::Model>) -> Result<PostResponse, (StatusCode, &'static str)> {
        let form = form.0;

        PostOrmMutation::create_post(&state.conn, form)
            .await
            .expect("could not insert post");

        let data = FlashData {
            kind: "success".to_owned(),
            message: "Post succcessfully added".to_owned(),
        };

        Ok(post_response(&mut cookies, data))
    }

    pub async fn edit_post(state: State<AppState>, Path(id): Path<String>) -> Result<Html<String>, (StatusCode, &'static str)> {
        let id = to_object_id(id.clone()).unwrap();

        let post: post::Model = PostOrmQuery::find_post_by_id(&state.conn, id)
            .await
            .expect("could not find post")
            .unwrap_or_else(|| panic!("could not find post with id {id}"));

        let mut ctx = tera::Context::new();
        ctx.insert("post", &post);

        let body = state
            .templates
            .render("edit.html.tera", &ctx)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

        Ok(Html(body))
    }

    pub async fn update_post(state: State<AppState>, Path(id): Path<String>, mut cookies: Cookies, form: Form<post::Model>) -> Result<PostResponse, (StatusCode, String)> {
        let form = form.0;
        // let id = to_object_id(id.clone()).unwrap();
        let id = to_object_id(id.clone()).unwrap();

        PostOrmMutation::update_post_by_id(&state.conn, id, form)
            .await
            .expect("could not edit post");

        let data = FlashData {
            kind: "success".to_owned(),
            message: "Post succcessfully updated".to_owned(),
        };

        Ok(post_response(&mut cookies, data))
    }

    pub async fn delete_post(state: State<AppState>, Path(id): Path<String>, mut cookies: Cookies) -> Result<PostResponse, (StatusCode, &'static str)> {
        let id = to_object_id(id.clone()).unwrap();

        PostOrmMutation::delete_post(&state.conn, id)
            .await
            .expect("could not delete post");

        let data = FlashData {
            kind: "success".to_owned(),
            message: "Post succcessfully deleted".to_owned(),
        };

        Ok(post_response(&mut cookies, data))
    }
}


