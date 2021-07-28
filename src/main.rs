mod dbs;
mod models;
mod repositories;
mod settings;

use dbs::mongo::Connection;
use models::Account;
use repositories::AccountRepository;
use settings::Settings;

use serde::{Deserialize, Serialize};
use tide::{Request, Response, prelude::*}; // Pulls in the json! macro.
                      // use tide::{Body, Request};
use futures::{StreamExt, stream::TryStreamExt};
use mongodb::{bson::doc, options::FindOptions};

// Shared application state
#[derive(Clone)]
struct State {
    pub db: Connection,
}

impl State {
    /// Create a new instance of `State`.
    pub async fn new(db: Connection) -> Self {
      Self { db }
    }
    /// Access the mongodb connection.
    pub fn mongo(&self) -> &Connection {
      &self.db
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {

    // Getting settings from config files
    let settings = Settings::new()?;

    tide::log::start();

    // MongoDB database connection
    let connection = Connection::new(
      settings.database_address(),
      settings.database_port(),
      settings.database_name(),
    ).await;

    // Create Tide App with state
    let mut app = tide::with_state(State::new(connection).await);

    // Tide routes
    app.at("/").get(get_accounts);

    // Tide App listener
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn get_accounts(mut req: Request<State>) -> tide::Result<impl Into<Response>> {
  let mut accs : Vec<Account> = Vec::new();

  // Repositories initialization
  let account_repository = AccountRepository::new(req.state().db.database().await, "accounts".to_string());

  // Test with MongoDB collections
  let coll = account_repository.get_collection().await;
  let mut cursor = coll.find(doc! {}, FindOptions::builder().build()).await?;

  while let Some(result) = cursor.next().await {
    match result {
      Ok(account) => accs.push(account),
      Err(error) => print!("Cannot get accounts"),
    }
  }

  println!("Hello from main");
  Ok(json!({
      "users": accs
  }))
}