use mongodb::{Collection, Database, bson::Document};

use crate::{dbs::mongo::Connection, models::Account};

pub struct AccountRepository {
  database: Database,
  collection: String,
}

impl AccountRepository {
    pub fn new(database: Database, collection: String) -> Self {
      Self {
        database,
        collection,
      }
    }

    pub async fn get_collection(&self) -> Collection<Account> {
      self.database.collection::<Account>(&self.collection.to_string())
    }
}