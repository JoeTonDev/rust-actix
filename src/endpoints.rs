use actix_web::{get, post, delete, put, HttpResponse, Responder, web};
use crate::database;
use crate::database::insert_flight_plan;
use crate::schema::{FlightPlan, User};

#[post("/api/v1/admin/user/create")]
pub async fn new_user(user: web::Json<User>) -> impl Responder {
  match database::create_user(user.into_inner()) {
    Ok(api_key) => HttpResponse::Ok().body(api_key),
    Err(e) => HttpResponse::InternalServerError().body(e.to_string())
  }
}

#[get("/api/v1/flightplan")]
pub async fn get_all_flight_plans() -> impl Responder {
  match database::get_all_flight_plans().unwrap() {
     Some(flight_plan_list) => {
          return HttpResponse::Ok().content_type("application/json").json(flight_plan_list);
     },
     None => {
          return HttpResponse::NoContent().body("There are no flight plans filed with this system");
     }
  }
}