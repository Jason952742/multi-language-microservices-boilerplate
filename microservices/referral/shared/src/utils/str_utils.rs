use chrono::{NaiveDateTime};
use uuid::{Uuid};

pub fn opt_to_uuid(uuid_str: Option<String>) -> Uuid {
    Uuid::parse_str(&uuid_str.unwrap()).unwrap()
}

pub fn convert_to_i32(num_str: Option<String>) -> i32 {
    num_str.unwrap().parse().unwrap()
}

pub fn convert_to_bool(bool_str: Option<String>) -> bool {
    bool_str.unwrap().parse().unwrap()
}

pub fn string_opt_to_datetime_opt(opt: &Option<String>) -> Option<NaiveDateTime> {
    opt.as_ref().map(|v| NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").unwrap())
}

pub fn string_to_datetime(s: Option<String>) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(&s.unwrap(), "%Y-%m-%d %H:%M:%S%.f").unwrap()
}