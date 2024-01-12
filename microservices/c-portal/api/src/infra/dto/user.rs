use serde_derive::{Deserialize, Serialize};
use validator::Validate;
use crate::domain::entities::cache_user::CacheUser;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateBody {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    pub identifier: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password:String,
    #[serde(default)]
    pub referral_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AuthorizeBody {
    #[validate(length(min = 3, message = "username must be at least 3 characters"))]
    pub identifier: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password:String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PasswordBody {
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub old_password:String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub new_password:String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub user: CacheUser,
}
