use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
  pub database_url: String,
  pub debug: bool,

}

impl Default for Database {
  fn default() -> Self {
    Database {
      database_url: String::from("../database.sql"),
      debug: true
    }
  }
}