use bson::oid::ObjectId;
use mongodb::bson::{self, doc, Document};
use mongodb::error::Result;
use mongodb::sync::{Collection, Cursor, Database};
use serde::Deserialize;

use crate::models::book_instance::BookInstance;
use crate::models::expanded_book_instance::ExpandedBookInstance;

pub struct BookInstanceCollection {
    book_instance_coll: Collection<Document>,
}

#[derive(Deserialize)]
struct CountDoc {
    count: u64,
}

impl BookInstanceCollection {
    pub fn count_book_instances(&self) -> Result<u64> {
        self.book_instance_coll.estimated_document_count(None)
    }

    pub fn count_books_by_status(&self, status: &str) -> u64 {
        let mut results: Cursor<Document> = self
            .book_instance_coll
            .aggregate(
                vec![
                    doc! {
                        "$match": {
                            "status": status.to_string(), //TODO AKIRA: make this an enum
                        }
                    },
                    doc! {
                        "$count": "count",
                    },
                ],
                None,
            )
            .unwrap();

        //TODO AKIRA: handle this in a better way
        match results.next() {
            Some(d) => bson::from_document::<CountDoc>(d.unwrap()).unwrap().count,
            None => panic!("No book instance collections could be counted by status {}. Suggested error: the collection string is wrong", status),
        }
    }

    pub fn build(db: &Database) -> BookInstanceCollection {
        BookInstanceCollection {
            book_instance_coll: db.collection::<Document>("bookinstances"),
        }
    }

    pub fn list_book_instances(&self) -> Vec<ExpandedBookInstance> {
        let cursor = match self.book_instance_coll.aggregate(
            vec![
                doc! {
                    "$lookup": {
                        "from": "books",
                        "localField": "book",
                        "foreignField": "_id",
                        "as": "book_obj",
                    }
                },
                doc! {
                    "$sort": {
                        "book_obj.title" : 1,
                    }
                },
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_book_instancess");

        let res_documents: Vec<Result<Document>> = cursor.collect::<Vec<Result<Document>>>();
        let mut documents: Vec<Document> = Vec::new();
        for res in res_documents {
            let document = match res {
                Ok(r) => r,
                Err(e) => panic!("Error getting document in list_books: {:?}", e),
            };
            documents.push(document);
        }
        let mut book_instances: Vec<ExpandedBookInstance> = Vec::new();
        println!("Found {:?} books", documents.len());
        for d in documents {
            let expanded_book_instance = match bson::from_document::<ExpandedBookInstance>(d) {
                Ok(b) => b,
                Err(e) => panic!("Error deserializing expanded book instance {:?}", e),
            };
            book_instances.push(expanded_book_instance);
        }
        book_instances
    }

    pub fn get_book_instance_by_book(&self, book_id: &str) -> Vec<BookInstance> {
        let book_object_id = ObjectId::parse_str(book_id).unwrap();
        let cursor = self
            .book_instance_coll
            .find(doc! { "book" : book_object_id }, None)
            .unwrap();
        let res_documents = cursor.collect::<Vec<Result<Document>>>();
        let mut documents: Vec<Document> = Vec::new();
        for res in res_documents {
            let document = match res {
                Ok(r) => r,
                Err(e) => panic!("Error getting document in list_books: {:?}", e),
            };
            documents.push(document);
        }
        let mut book_instances: Vec<BookInstance> = Vec::new();
        for d in documents {
            println!("*** Document: {:?}", &d);
            let book_instance = match bson::from_document::<BookInstance>(d) {
                Ok(b) => b,
                Err(e) => panic!(
                    "Error getting document in get_book_instance_by_book: {:?}",
                    e
                ),
            };
            book_instances.push(book_instance);
        }
        book_instances
    }
}
