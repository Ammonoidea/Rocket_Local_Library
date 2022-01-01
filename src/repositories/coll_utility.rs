use bson::Document;
use mongodb::error::Result;
use mongodb::sync::Cursor;
use rocket::serde::DeserializeOwned;

pub fn doc_vec_converter<T: DeserializeOwned>(cursor: Cursor<Document>) -> Vec<T> {
    let res_documents = cursor.collect::<Vec<Result<Document>>>();
    let mut documents: Vec<Document> = Vec::new();
    for res in res_documents {
        let document = match res {
            Ok(r) => r,
            Err(e) => panic!("Error getting document: {:?}", e),
        };
        documents.push(document);
    }
    let mut converted: Vec<T> = Vec::new();
    for d in documents {
        let conv = match bson::from_document::<T>(d) {
            Ok(t) => t,
            Err(e) => panic!("Error deserializing {} {:?}", std::any::type_name::<T>(), e),
        };
        converted.push(conv);
    }
    converted
}
