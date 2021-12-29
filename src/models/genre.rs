use serde::Deserialize;
use bson::oid::ObjectId;

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub _id: ObjectId,
    pub name: String,
}
