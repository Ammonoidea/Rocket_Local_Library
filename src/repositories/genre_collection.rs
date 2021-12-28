use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

pub struct GenreCollection {
    genre_coll: Collection<Document>,
}

impl GenreCollection {
    pub fn count_genres(&self) -> Result<u64> {
        self.genre_coll.estimated_document_count(None)
    }

    pub fn build(db: &Database) -> GenreCollection {
        GenreCollection {
            genre_coll: db.collection::<Document>("genres"),
        }
    }
}
