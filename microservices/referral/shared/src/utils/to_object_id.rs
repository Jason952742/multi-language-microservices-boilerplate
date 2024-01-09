use std::str::FromStr;
use bson::oid::ObjectId;
use bson::ser::Error;
use uuid::Uuid;

pub fn to_object_id<S: AsRef<str>>(id: S) -> Result<ObjectId, Error> {
    ObjectId::parse_str(id.as_ref()).map_err(|_e| Error::InvalidCString("id error".to_string()))
}

pub fn uuid_to_bson_uuid(id: Uuid) -> bson::Uuid {
    bson::Uuid::parse_str(id.to_string()).unwrap()
}

pub fn bson_uuid_to_uuid(id: bson::Uuid) -> Uuid {
    Uuid::from_str(id.to_string().as_str()).unwrap()
}
