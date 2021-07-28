use serde::{Deserialize, Serialize};
use mongodb::{bson::oid::ObjectId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
  #[serde(rename = "_id")]
  id: ObjectId,
}

impl Account {
  pub fn new() -> Self {
    Self { id: ObjectId::new() }
  }

  pub fn get_id(&self) -> ObjectId {
    self.id
  }
}