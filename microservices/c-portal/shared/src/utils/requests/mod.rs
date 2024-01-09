pub mod form_validate;
pub mod version;
pub mod path_validate;
pub mod json_validate;
pub mod query_validate;
pub mod pagination;

use std::fmt;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};
pub use form_validate::*;
pub use version::*;
pub use path_validate::*;
pub use json_validate::*;
pub use query_validate::*;
pub use pagination::*;

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
