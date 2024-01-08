#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::{Request};
use rocket_dyn_templates::Template;
use serde_json::json;
use shared::{Config};
use crate::application::rest::{create, delete, destroy, edit, health_check, list, new, update};

mod infra;
mod domain;
mod application;

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
        .mount("/", routes![health_check])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await
        .map(|_| ())?;

    println!("Rocket: deorbit.");

    Ok(())
}
