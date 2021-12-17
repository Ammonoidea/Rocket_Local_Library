use crate::models::documentable::Persistable;
use mongodb::bson::{doc, Document};

#[derive(Debug, PartialEq, Eq)]
pub struct Genre {
    pub name: String,
}

impl Persistable for Genre {
    fn to_document(&self) -> Document {
        doc! {
            "name": self.name.clone()
        }
    }

    fn from_document(document: &Document) -> Self {
        unimplemented!();
    }

    fn coll_name() -> String {
        unimplemented!();
    }
}
