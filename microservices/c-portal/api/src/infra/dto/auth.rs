use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthBody {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct TokenBody {
    access_token: String,
    token_type: String,
}

impl TokenBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
