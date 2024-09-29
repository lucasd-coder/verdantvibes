use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;
use tracing::error;

use crate::domain::event::service::create_event;
use crate::domain::event::EventDTO;
pub type APIResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[actix_web::post("/event")]
pub async fn salve(pool: web::Data<PgPool>, payload: web::Json<EventDTO>) -> APIResult {
    let dto = EventDTO {
        name: payload.name.clone(),
        description: payload.description.clone(),
        location: payload.location.clone(),
        start_date: payload.start_date,
        end_date: payload.end_date,
        organizer_id: payload.organizer_id,
        max_participants: payload.max_participants,
        sustainable_practice: payload.sustainable_practice.clone(),
    };

    match create_event(&pool, dto).await {
        Ok(()) => Ok(HttpResponse::Created()
            //.insert_header(("Location", format!("/events/{}", event_id)))
            .finish()),
        Err(e) => {
            error!("Failed to create event: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
