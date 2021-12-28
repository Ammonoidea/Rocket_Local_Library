use mongodb::bson;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

use crate::models::expanded_book::ExpandedBook;

pub struct BookCollection {
    book_coll: Collection<Document>,
}

impl BookCollection {
    pub fn count_books(&self) -> Result<u64> {
        self.book_coll.estimated_document_count(None)
    }

    pub fn list_books(&self) -> Vec<ExpandedBook> {
        let cursor = match self.book_coll.aggregate(
            vec![
                doc! {
                    "$lookup": {
                        "from": "authors",
                        "localField": "author",
                        "foreignField": "_id",
                        "as": "author_obj",
                    }
                },
                doc! {
                    "$sort": {
                        "title" : 1,
                    }
                },
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_books");

        let res_documents: Vec<Result<Document>> = cursor.collect::<Vec<Result<Document>>>();
        let mut documents: Vec<Document> = Vec::new();
        for res in res_documents {
            let document = match res {
                Ok(r) => r,
                Err(e) => panic!("Error getting document in list_books: {:?}", e),
            };
            documents.push(document);
        }
        let mut books: Vec<ExpandedBook> = Vec::new();
        println!("Found {:?} books", documents.len());
        for d in documents {
            println!("Document: {:?}", &d);
            let expanded_book = match bson::from_document::<ExpandedBook>(d) {
                Ok(b) => b,
                Err(e) => panic!("Error deserializing expanded book {:?}", e),
            };
            books.push(expanded_book);
        }
        books
    }

    pub fn build(db: &Database) -> BookCollection {
        BookCollection {
            book_coll: db.collection::<Document>("books"),
        }
    }
}
