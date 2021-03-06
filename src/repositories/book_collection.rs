use bson::oid::ObjectId;
use mongodb::bson;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::sync::{Collection, Database};

use crate::models::book::Book;
use crate::models::expanded_book::ExpandedBook;
use crate::models::expanded_book_with_genre::ExpandedBookWithGenre;

use super::coll_utility;

pub struct BookCollection {
    book_coll: Collection<Document>,
}

impl BookCollection {
    pub fn count_books(&self) -> Result<u64> {
        self.book_coll.estimated_document_count(None)
    }

    pub fn get_book_by_id(&self, id: &str) -> Option<ExpandedBookWithGenre> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let mut cursor = match self.book_coll.aggregate(
            vec![
                doc! {
                    "$match": {
                        "_id": object_id,
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "authors",
                        "localField": "author",
                        "foreignField": "_id",
                        "as": "author_obj",
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "genres",
                        "localField": "genre",
                        "foreignField": "_id",
                        "as": "genre_objs",
                    }
                },
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return None,
        };

        cursor
            .next()
            .map(|d| bson::from_document::<ExpandedBookWithGenre>(d.unwrap()).unwrap())
    }

    pub fn list_books(&self) -> Vec<ExpandedBook> {
        let cursor = match self.book_coll.aggregate(
            vec![
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
                        "title" : 1,
                    }
                },
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };

        println!("!!! Got cursor in list_books");

        coll_utility::doc_vec_converter::<ExpandedBook>(cursor)
    }

    pub fn get_books_by_genre(&self, genre_id: &str) -> Vec<Book> {
        let object_id = ObjectId::parse_str(genre_id).unwrap();
        let cursor = match self.book_coll.aggregate(
            vec![
                doc! { "$match": { "genre": object_id}},
                doc! {"$sort": {"title": 1}},
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };
        coll_utility::doc_vec_converter::<Book>(cursor)
    }

    pub fn get_books_by_author(&self, author_id: &str) -> Vec<Book> {
        let object_id = ObjectId::parse_str(author_id).unwrap();
        let cursor = match self.book_coll.aggregate(
            vec![
                doc! { "$match": { "author": object_id}},
                doc! {"$sort": {"title": 1}},
            ],
            None,
        ) {
            Ok(cursor) => cursor,
            Err(_) => return vec![],
        };
        coll_utility::doc_vec_converter::<Book>(cursor)
    }

    pub fn build(db: &Database) -> BookCollection {
        BookCollection {
            book_coll: db.collection::<Document>("books"),
        }
    }
}
