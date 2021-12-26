use mongodb::bson::{doc, Document, from_document};
use mongodb::bson;
use mongodb::error::Result;
use mongodb::Cursor;
use mongodb::{Collection, Database};
use futures::TryStreamExt;
use futures::stream::TryCollect;

use serde::Deserialize;

use crate::models::author::Author;

pub struct BookCollection {
    book_coll: Collection<Document>,
}

#[derive(Deserialize)]
struct ExpandedBook {
    pub title: String,
    pub author: String,
    pub summary: String,
    pub isbn: String,
    pub genre: Vec<String>,
    pub author_obj: Author,
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

        let mut documents: Vec<Document> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
        documents.iter().map(|d: &Document| bson::from_document::<ExpandedBook>(*d).unwrap())
        .collect::<Vec<_>>()
    }

    pub fn build(db: &Database) -> BookCollection {
        BookCollection {
            book_coll: db.collection::<Document>("books"),
        }
    }
}
