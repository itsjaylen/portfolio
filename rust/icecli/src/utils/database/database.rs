use diesel::prelude::*;
use dotenvy::dotenv;
use diesel::pg::PgConnection;

use std::env;

use crate::utils::config::app_config::AppConfig;



pub fn establish_connection() -> PgConnection {
  let app_config = AppConfig::load_config();
  dotenv().ok();

  let database_url = &app_config.database.database_url;

  if database_url.is_empty() {
      panic!("DATABASE_URL must be set");
  }

  Connection::establish(database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

