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

pub fn get_all_flight_plans() -> Result<Option<Vec<FlightPlan>>, Box<dyn Error>> {
  let mut flight_plan_list : Vec<FlightPlan> = Vec::new();

  let connection = get_database_connection()?;
  let mut statement = connection.prepare("SELECT * FROM flight_plan")?;
  let query_result = statement.query_map([], |row| {
      Ok(FlightPlan {
          flight_plan_id: row.get(1)?,
          altitude: row.get(2)?,
          airspeed: row.get(3)?,
          aircraft_identification: row.get(4)?,
          aircraft_type: row.get(5)?,
          arrival_airport: row.get(6)?,
          departing_airport: row.get(7)?,
          flight_type: row.get(8)?,
          departure_time: row.get(9)?,
          estimated_arrival_time: row.get(10)?,
          route: row.get(11)?,
          remarks: row.get(12)?,
          fuel_hours: row.get(13)?,
          fuel_minutes: row.get(14)?,
          number_onboard: row.get(15)?
      })
  })?;

  for plan in query_result {
      flight_plan_list.push(plan?);
  }

  match flight_plan_list.len() > 0 {
      true => {
          Ok(Some(flight_plan_list))
      }
      false => {
          Ok(None)
      }
  }
}

pub fn get_flight_plan_by_id(plan_id: String) -> Result<Option<FlightPlan>, Box<dyn Error>> {
  let connection = get_database_connection()?;
  let mut statement = connection.prepare("SELECT * FROM flight_plan WHERE flight_plan_id = ?1")?;
  let query_result = statement.query_map([&plan_id], |row| {
      Ok(FlightPlan {
          flight_plan_id: row.get(1)?,
          altitude: row.get(2)?,
          airspeed: row.get(3)?,
          aircraft_identification: row.get(4)?,
          aircraft_type: row.get(5)?,
          arrival_airport: row.get(6)?,
          departing_airport: row.get(7)?,
          flight_type: row.get(8)?,
          departure_time: row.get(9)?,
          estimated_arrival_time: row.get(10)?,
          route: row.get(11)?,
          remarks: row.get(12)?,
          fuel_hours: row.get(13)?,
          fuel_minutes: row.get(14)?,
          number_onboard: row.get(15)?
      })
  })?;

  let mut flight_plan: Option<FlightPlan> = None;

  for plan in query_result {
      flight_plan = Some(plan?);
      break;
  }

  Ok(flight_plan)

}

pub fn delete_flight_plan(plan_id: String) -> Result<bool, Box<dyn Error>> {
  let mut successful = false;
  let connection = get_database_connection()?;
  let mut statement = connection.prepare("DELETE FROM flight_plan WHERE flight_plan_id = ?1")?;
  let query_result = statement.execute([&plan_id])?;
  if query_result > 0 {
      successful = true;
  }
  Ok(successful)
}

