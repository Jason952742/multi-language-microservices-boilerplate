use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, FromForm, Deserialize)]
pub struct Login {
    mold: IdentityMold,
    #[field(validate = len(3..12))]
    identifier: String,
    #[field(validate = len(6..24))]
    password:String,
    #[allow(unused)]
    remember_me: bool,
}

#[derive(Debug, FromForm)]
pub struct Register {
    #[field(validate = len(3..12))]
    identifier: String,
    #[field(validate = len(6..24))]
    password: String,
}

#[derive(Debug, Deserialize, FromFormField)]
pub enum IdentityMold {
    Username,
    Phone,
    Email,
    WeChat,
    Facebook,
    IdentityCard,
    Passport,
    BusinessLicense,
    EvmWallet,
    Cosmos,
    Bitcoin,
}