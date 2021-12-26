use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::{Collection, Database};

pub struct GenreCollection {
    genre_coll: Collection<Document>,
}

impl GenreCollection {
    pub async fn count_genres(&self) -> Result<u64> {
        self.genre_coll.estimated_document_count(None).await
    }

    pub fn build(db: &Database) -> GenreCollection {
        GenreCollection {
            genre_coll: db.collection::<Document>("genres"),
        }
    }
}
