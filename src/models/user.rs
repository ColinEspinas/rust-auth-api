use serde::{Deserialize, Serialize};
use mongodb::{bson::oid::ObjectId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  #[serde(rename = "_id")]
  id: ObjectId,
  name: String,
  email: String,
  password: String,
}

impl User {
  pub fn new(name: String, email: String, password: String) -> Self {
    Self { id: ObjectId::new(), name, email, password }
  }
}