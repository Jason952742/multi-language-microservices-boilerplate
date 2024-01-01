use bson::oid::ObjectId;
use bson::ser::Error;

pub fn to_object_id<S: AsRef<str>>(id: S) -> Result<ObjectId, Error> {
  ObjectId::parse_str(id.as_ref()).map_err(|e| Error::InvalidCString("id error".to_string()))
}
