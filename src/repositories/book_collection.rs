use mongodb::bson::{doc, Document};
use mongodb::bson;
use mongodb::error::Result;
use mongodb::{Collection, Database};
use futures::TryStreamExt;

use crate::models::expanded_book::ExpandedBook;

pub struct BookCollection {
    book_coll: Collection<Document>,
}


impl BookCollection {
    pub async fn count_books(&self) -> Result<u64> {
        self.book_coll.estimated_document_count(None).await
    }

    pub async fn list_books(&self) -> Vec<ExpandedBook> {
        let cursor = match self.book_coll.aggregate(vec![
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
                        "title" : "1"
                    }
                },
            ],
            None
        ).await {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        let documents: Vec<Document> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
        let mut books: Vec<ExpandedBook> = Vec::new();
        for d in documents {
            books.push(bson::from_document::<ExpandedBook>(d).unwrap());
        }
        books
    }

    pub fn build(db: &Database) -> BookCollection {
        BookCollection {
            book_coll: db.collection::<Document>("books"),
        }
    }
}
