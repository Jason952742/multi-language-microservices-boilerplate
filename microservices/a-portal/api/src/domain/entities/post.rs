use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use shared::mongodb::bson;
use shared::mongodb::bson::Bson;
use shared::mongodb::bson::oid::ObjectId;
use shared::to_object_id;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct FormModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub text: String,
}

impl Into<Model> for FormModel {
    fn into(self) -> Model {
        Model {
            id: self.id.map(|x| to_object_id(&x).unwrap()),
            title: self.title,
            text: self.text
        }
    }
}

impl From<Model> for FormModel {
    fn from(value: Model) -> Self {
        Self {
            id: value.id.map(|x| x.to_string()),
            title: value.title,
            text: value.text,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub text: String,
}

impl Into<Bson> for Model {
    fn into(self) -> Bson {
        Bson::from(bson::to_document(&self).unwrap())
    }
}
