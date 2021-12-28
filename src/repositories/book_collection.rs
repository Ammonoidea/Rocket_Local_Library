use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::{Collection, Database};

use crate::models::expanded_book::ExpandedBook;

pub struct BookCollection {
    book_coll: Collection<Document>,
}

impl BookCollection {
    pub async fn count_books(&self) -> Result<u64> {
        self.book_coll.estimated_document_count(None).await
    }

    pub async fn list_books(&self) -> Vec<ExpandedBook> {
        let cursor = match self
            .book_coll
            .aggregate(
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
            )
            .await
        {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_books");

        let documents: Vec<Document> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
        let mut books: Vec<ExpandedBook> = Vec::new();
        println!("Found {:?} books", documents.len());
        for d in documents {
            let expanded_book = match bson::from_document::<ExpandedBook>(d) {
                Ok(b) => b,
                Err(e) => panic!("Error deserliazing expanded book {:?}", e),
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
