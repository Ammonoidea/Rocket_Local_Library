use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

pub struct AuthorCollection {
    author_coll: Collection<Document>,
}

impl AuthorCollection {
    pub fn count_authors(&self) -> Result<u64> {
        self.author_coll.estimated_document_count(None)
    }

    pub fn build(db: &Database) -> AuthorCollection {
        AuthorCollection {
            author_coll: db.collection::<Document>("authors"),
        }
    }
}
