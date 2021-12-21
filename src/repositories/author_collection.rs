use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::{Collection, Database};

pub struct AuthorCollection {
    author_coll: Collection<Document>,
}

impl AuthorCollection {
    pub async fn count_authors(&self) -> Result<u64> {
        self.author_coll.estimated_document_count(None).await
    }

    pub fn build(db: &Database) -> AuthorCollection {
        AuthorCollection {
            author_coll: db.collection::<Document>("authors"),
        }
    }
}
