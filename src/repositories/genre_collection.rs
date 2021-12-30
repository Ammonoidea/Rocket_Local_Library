use bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

use crate::models::genre::Genre;

pub struct GenreCollection {
    genre_coll: Collection<Document>,
}

impl GenreCollection {
    pub fn count_genres(&self) -> Result<u64> {
        self.genre_coll.estimated_document_count(None)
    }

    pub fn get_genre_by_id(&self, id: &str) -> Option<Genre> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let mut cursor = self
            .genre_coll
            .find(doc! { "_id" : object_id }, None)
            .unwrap();
        cursor
            .next()
            .map(|d| bson::from_document::<Genre>(d.unwrap()).unwrap())
    }

    pub fn list_genres(&self) -> Vec<Genre> {
        let cursor = match self
            .genre_coll
            .aggregate(vec![doc! {"$sort": {"name" : 1,}}], None)
        {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_genres");

        let res_documents: Vec<Result<Document>> = cursor.collect::<Vec<Result<Document>>>();
        let mut documents: Vec<Document> = Vec::new();
        for res in res_documents {
            let document = match res {
                Ok(r) => r,
                Err(e) => panic!("Error getting document in list_genres: {:?}", e),
            };
            documents.push(document);
        }
        let mut genres: Vec<Genre> = Vec::new();
        println!("Found {:?} genres", documents.len());
        for d in documents {
            let genre = match bson::from_document::<Genre>(d) {
                Ok(b) => b,
                Err(e) => panic!("Error deserializing genre {:?}", e),
            };
            genres.push(genre);
        }
        genres
    }

    pub fn build(db: &Database) -> GenreCollection {
        GenreCollection {
            genre_coll: db.collection::<Document>("genres"),
        }
    }
}
