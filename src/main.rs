mod models;
mod dbs;
mod repositories;
mod settings;

use dbs::{mongo::Connection};
use repositories::AccountRepository;
use models::{Account};
use settings::Settings;

use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.
// use tide::{Body, Request};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};


#[async_std::main]
async fn main() -> tide::Result<()> {
    let settings = Settings::new();

    tide::log::start();
    let mut app = tide::new();
    let connection = Connection::new(&settings?.database_name()).await;
    let account_repository = AccountRepository::new(connection.database().await, "accounts".to_string());

    let coll = account_repository.get_collection().await;

    let mut cursor = coll.find(doc! {}, FindOptions::builder().build()).await?;

    while let Some(result) = cursor.try_next().await? {
        println!("Result: {}", result.get_id())
    }

    app.at("/").get(|_| async {

        let acc = Account::new();

        println!("Hello from main");

        Ok(json!({
            "users": []
        }))
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}