use actix_web::HttpResponse;

pub mod event_controller;
pub mod user_controller;

pub type APIResult = Result<HttpResponse, Box<dyn std::error::Error>>;
