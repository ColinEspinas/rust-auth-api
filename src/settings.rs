use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct MongoDatabase {
    name: String,
    address: String,
    port: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    database: MongoDatabase,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Now that we're done, let's access our configuration
        // println!("database: {:?}", s.get::<String>("database.name"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }

    pub fn database_name(&self) -> &str {
      self.database.name.as_str()
    }

    pub fn database_address(&self) -> &str {
      self.database.address.as_str()
    }

    pub fn database_port(&self) -> &str {
      self.database.port.as_str()
    }
}