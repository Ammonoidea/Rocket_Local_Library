use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

pub struct BookCollection {
    book_coll: Collection<Document>,
}

impl BookCollection {
    pub fn count_books(&self) -> Result<u64> {
        self.book_coll.estimated_document_count(None)
    }

    pub fn build(db: &Database) -> BookCollection {
        BookCollection {
            book_coll: db.collection::<Document>("books"),
        }
    }
}
