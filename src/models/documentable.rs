use mongodb::bson::Document;

pub trait Persistable {
    fn to_document(&self) -> Document;
    fn from_document(document: &Document) -> Self;
    fn coll_name() -> String;
}