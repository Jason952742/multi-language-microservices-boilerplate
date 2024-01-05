use chrono::{NaiveDateTime};
use chrono::{DateTime, Utc};
use neo4rs::DeError;
use uuid::{Uuid};
use base64::{Engine};
use base64::engine::general_purpose;

pub fn to_uuid(str: &str) -> Uuid {
    str.parse().unwrap()
}

pub fn opt_to_uuid(uuid_str: Result<String, DeError>) -> Uuid {
    Uuid::parse_str(&uuid_str.unwrap()).unwrap()
}

pub fn convert_to_i32(num_str: Result<String, DeError>) -> i32 {
    num_str.unwrap().parse().unwrap()
}

pub fn convert_to_bool(bool_str: Result<String, DeError>) -> bool {
    bool_str.unwrap().parse().unwrap()
}

pub fn string_opt_to_datetime_opt(opt: &Option<String>) -> Option<NaiveDateTime> {
    opt.as_ref().map(|v| NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").unwrap())
}

pub fn string_to_datetime(s: Result<String, DeError>) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&s.unwrap(), "%Y-%m-%d %H:%M:%S%.f").unwrap()
}

pub fn uuid_to_base64(id: Uuid) -> String {
    let orig = id.as_bytes();
    general_purpose::URL_SAFE_NO_PAD.encode(orig)
}

pub fn base64_to_uuid(encoded: String) -> Result<Uuid, base64::DecodeError> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(encoded)?;
    Ok(Uuid::from_slice(&*decoded).unwrap())
}

pub fn to_datetime(datetime_str: &str) -> DateTime<Utc> {
    let datetime = DateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S%.fZ")
        .expect("Failed to parse datetime string")
        .with_timezone(&Utc);
    datetime
}

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();
    let encoded = uuid_to_base64(id);

    println!("{}", encoded);

    let old_id = base64_to_uuid(encoded)?;

    assert_eq!(old_id, id);

    Ok(())
}