use mongodb::{Client, options::ClientOptions, Database};

#[derive(Clone)]
pub struct Connection {
  client: Client,
  database: Database,
}

impl Connection {
  pub async fn client(&self) -> Client {
    self.client.clone()
  }

  pub async fn database(&self) -> Database {
    self.database.clone()
  }

  pub async fn new(address: &str, port: &str, database_name: &str) -> Connection {

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(format!("mongodb://{}:{}", address, port)).await.expect("Failed to parse options!");

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options).expect("Failed to initialize database!");

    // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await.unwrap() {
    //     println!("{}", db_name);
    // }
    let database = client.database(database_name);

    Connection { client: client, database }
  }
}
