use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub _id: ObjectId,
    pub name: String,
}
