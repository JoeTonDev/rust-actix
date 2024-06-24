use actix_web::{get, post, delete, put, HttpResponse, Responder, web};
use crate::database;
use crate::database::insert_flight_plan;
use crate::schema::{FlightPlan, User};

