use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,

    #[serde(skip_serializing_if = "is_empty")]
    pub message: Option<String>,
}

impl Response {
    pub fn http_response(&self) -> HttpResponse {
        match self.status {
            Some(200) => HttpResponse::Ok().json(self),
            Some(201) => HttpResponse::Created().json(self),
            Some(400) => HttpResponse::BadRequest().json(self),
            Some(401) => HttpResponse::Unauthorized().json(self),
            Some(403) => HttpResponse::Forbidden().json(self),
            Some(404) => HttpResponse::NotFound().json(self),
            Some(500) => HttpResponse::InternalServerError().json(self),
            _ => HttpResponse::InternalServerError().json(Response {
                status: Some(500),
                message: Some(String::from("Internal Server Error")),
            }),
        }
    }
}

fn is_empty(val: &Option<String>) -> bool {
    match val {
        Some(s) => s.is_empty(),
        None => true,
    }
}
