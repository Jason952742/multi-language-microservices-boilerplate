use rocket::time::Date;
use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context, Strict};
use rocket::fs::{FileServer, TempFile, relative};
use crate::infra::dto::Login;

#[post("/login", data = "<form>")]
pub async fn login(form: Form<Strict<Login>>) -> String {
    println!("{:?}", form);

    "OK".to_string()
}
