use config::Config;
use std::error::Error;
use rusqlite::Connection;
use uuid::Uuid;
use crate::schema::{FlightPlan, User};

pub fn create_user(user: User) -> Result<String, Box<dyn Error>> {
  let api_key = Uuid::new_v4().as_simple().to_string();    
  let connection = get_database_connection()?;
  let mut statement = connection.prepare("INSERT INTO users (full_name, api_key) VALUES (?, ?, ?, ?)")?;
  let _ = statement.execute((user.name, api_key.clone()))?;
  Ok(api_key)
}

pub fn get_user(api_key: String) -> Result<Option<User>, Box<dyn Error>> {
  let connection = get_database_connection()?;
  let mut statement = connection.prepare("SELECT * FROM users WHERE api_key = ?")?;
  let query_result = statement.query_map([&api_key], |row| {
      Ok(User {
          name: row.get(1)?,
          api_key: row.get(4)?
      })
  })?;

  let mut user: Option<User> = None;
  for api_user in query_result {
      user = Some(api_user.unwrap());
  }

  Ok(user)
}