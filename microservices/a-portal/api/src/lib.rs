#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::{Request};
use rocket_dyn_templates::Template;
use serde_json::json;
use shared::{Config};
use crate::application::rest::post_svc::{create, delete, destroy, edit, list, new, update};
use crate::application::rest::health_svc::health_check;
use crate::application::rest::auth_svc::login;

mod infra;
mod domain;
mod application;


pub async fn start(_config: Config) -> anyhow::Result<()> {
    rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![new, create, delete, destroy, list, edit, update])
        .mount("/", routes![health_check])
        .mount("/", routes![login])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await
        .map(|_| ())?;

    println!("Rocket: deorbit.");

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