use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;
use tracing::error;

use crate::domain::event::service::{EventServiceFactory, EventService, ServiceFactory};
use crate::{controller::APIResult, domain::event::EventDTO};

#[actix_web::post("/events")]
pub async fn save(pool: web::Data<PgPool>, payload: web::Json<EventDTO>) -> APIResult {
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

    let factory = EventServiceFactory;

    let event_service = factory.create_service(pool.get_ref().clone()).await?;
    match event_service.create_event(dto).await {
        Ok(r) => Ok(r.http_response()),
        Err(e) => {
            error!("Failed to create event: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
