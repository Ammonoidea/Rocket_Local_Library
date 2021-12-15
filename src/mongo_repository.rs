use mongodb::bson::{doc, Document};
use mongodb::{Client};

struct MongoRepository {
    client: HttpStream;
}

