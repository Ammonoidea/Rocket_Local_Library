use mongodb::bson::{self, doc, Document};
use mongodb::error::Result;
use mongodb::{Collection, Cursor, Database};
use serde::Deserialize;
use futures::stream::TryStreamExt;

pub struct BookInstanceCollection {
    book_instance_coll: Collection<Document>,
}

#[derive(Deserialize)]
struct CountDoc {
    count: u64,
}

impl BookInstanceCollection {
    pub async fn count_book_instances(&self) -> Result<u64> {
        self.book_instance_coll.estimated_document_count(None).await
    }

    pub async fn count_books_by_status(&self, status: &str) -> u64 {
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
            .await.unwrap();

        //TODO AKIRA: handle this in a better way
        match results.try_next().await.unwrap() {
            Some(d) => bson::from_document::<CountDoc>(d).unwrap().count,
            None => panic!("No book instance collections could be counted by status {}. Suggested error: the collection string is wrong", status),
        }
    }

    pub fn build(db: &Database) -> BookInstanceCollection {
        BookInstanceCollection {
            book_instance_coll: db.collection::<Document>("bookinstances"),
        }
    }
}
