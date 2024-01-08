#[macro_use]
extern crate rocket;

use rocket::form::{Context, Form};
use rocket::fs::{relative, FileServer};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::Template;
use serde_json::json;
use shared::bson::doc;
use shared::mongo::MongoPool;
use shared::{Config, to_object_id};

use crate::domain::entities::post;
use crate::infra::repositories::{PostDbMutation, PostDbQuery};

mod infra;
mod domain;

const DEFAULT_POSTS_PER_PAGE: u64 = 5;

#[get("/new")]
async fn new() -> Template {
    Template::render("new", &Context::default())
}

#[post("/", data = "<post_form>")]
async fn create(post_form: Form<post::FormModel>) -> Flash<Redirect> {
    let conn = MongoPool::conn().await.clone();

    let form = post_form.into_inner();

    PostDbMutation::create_post(&conn, form.into())
        .await
        .expect("could not insert post");

    Flash::success(Redirect::to("/"), "Post successfully added.")
}

#[post("/<id>", data = "<post_form>")]
async fn update(id: &str, post_form: Form<post::FormModel>) -> Flash<Redirect> {
    let conn = MongoPool::conn().await.clone();
    let id = to_object_id(id).unwrap();

    let form = post_form.into_inner();

    PostDbMutation::update_post_by_id(&conn, id, form.into())
        .await
        .expect("could not edit post");

    Flash::success(Redirect::to("/"), "Post successfully edited.")
}

#[get("/?<page>&<posts_per_page>")]
async fn list(page: Option<u64>, posts_per_page: Option<u64>, flash: Option<FlashMessage<'_>>) -> Template {
    let conn = MongoPool::conn().await.clone();

    // Set page number and items per page
    let page = page.unwrap_or(1);
    let posts_per_page = posts_per_page.unwrap_or(DEFAULT_POSTS_PER_PAGE);
    if page == 0 {
        panic!("Page number cannot be zero");
    }

    let filter = doc! { };
    let find_options = None;
    let (num_pages, posts) = PostDbQuery::find_posts_in_page(&conn, filter, find_options, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");
    let posts: Vec<post::FormModel> = posts.into_iter().map(|x| post::FormModel::from(x)).collect();

    Template::render(
        "index",
        json!({
            "page": page,
            "posts_per_page": posts_per_page,
            "num_pages": num_pages,
            "posts": posts,
            "flash": flash.map(FlashMessage::into_inner),
        }),
    )
}

#[get("/<id>")]
async fn edit(id: &str) -> Template {
    let conn = MongoPool::conn().await.clone();
    let id = to_object_id(id).unwrap();

    let post: post::Model = PostDbQuery::find_post_by_id(&conn, id)
        .await
        .expect("could not find post")
        .unwrap_or_else(|| panic!("could not find post with id {id}"));

    let post = post::FormModel::from(post);

    Template::render(
        "edit",
        json!({
            "post": post,
        }),
    )
}

#[delete("/<id>")]
async fn delete(id: String) -> Flash<Redirect> {
    let conn = MongoPool::conn().await.clone();
    let id = to_object_id(id.clone()).unwrap();

    PostDbMutation::delete_post(&conn, id)
        .await
        .expect("could not delete post");

    Flash::success(Redirect::to("/"), "Post successfully deleted.")
}

#[delete("/")]
async fn destroy() -> Result<(), rocket::response::Debug<String>> {
    let conn = MongoPool::conn().await.clone();

    PostDbMutation::delete_all_post(&conn)
        .await
        .expect("could not delete post");

    Ok(())
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        json!({
            "uri": req.uri()
        }),
    )
}


pub async fn start(config: Config) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![new, create, delete, destroy, list, edit, update])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await
        .map(|_| ())?;

    println!("Rocket: deorbit.");

    Ok(())
}
