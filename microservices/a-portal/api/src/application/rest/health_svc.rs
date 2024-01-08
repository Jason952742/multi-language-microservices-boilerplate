

#[get("/health")]
pub async fn health_check() -> String {
    "OK".to_string()
}
