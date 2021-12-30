use bson::doc;
use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

use crate::models::author::Author;

pub struct AuthorCollection {
    author_coll: Collection<Document>,
}

impl AuthorCollection {
    pub fn count_authors(&self) -> Result<u64> {
        self.author_coll.estimated_document_count(None)
    }

    pub fn list_authors(&self) -> Vec<Author> {
        let cursor = match self
            .author_coll
            .aggregate(vec![doc! {"$sort": {"family_name" : 1,}}], None)
        {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_authors");

        let res_documents: Vec<Result<Document>> = cursor.collect::<Vec<Result<Document>>>();
        let mut documents: Vec<Document> = Vec::new();
        for res in res_documents {
            let document = match res {
                Ok(r) => r,
                Err(e) => panic!("Error getting document in list_authors: {:?}", e),
            };
            documents.push(document);
        }
        let mut authors: Vec<Author> = Vec::new();
        println!("Found {:?} authors", documents.len());
        for d in documents {
            let author = match bson::from_document::<Author>(d) {
                Ok(b) => b,
                Err(e) => panic!("Error deserializing expanded author {:?}", e),
            };
            authors.push(author);
        }
        authors
    }

    pub fn build(db: &Database) -> AuthorCollection {
        AuthorCollection {
            author_coll: db.collection::<Document>("authors"),
        }
    }
}
