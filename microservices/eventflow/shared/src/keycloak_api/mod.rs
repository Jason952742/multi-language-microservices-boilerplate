pub mod admin;
pub mod openid;
pub mod urls;
pub mod model;

// use std::env;
use reqwest::{Client, ClientBuilder};
use tokio::sync::OnceCell;
use tracing::info;
pub use model::*;

use colored::Colorize;
use jsonwebtoken::{decode_header, errors::Error as JwtError};

static SESSION: OnceCell<Client> = OnceCell::const_new();

pub async fn client() -> &'static Client {
    SESSION
        .get_or_init(|| async {
            dotenvy::dotenv().ok();
            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build().expect("init client error");
            info!("{}", "REQWEST CONNECTED".color("magenta"));
            client
        })
        .await
}

pub fn jwt_decode(token: String) -> Result<jsonwebtoken::Header, JwtError> {
    decode_header(&token)
}