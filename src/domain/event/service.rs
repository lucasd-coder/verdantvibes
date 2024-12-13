use std::sync::Arc;

use crate::domain::event::repository::{EventRepository, Repository};
use crate::domain::response::Response;
use crate::domain::{event::Event, event::EventDTO};

use sqlx::postgres::PgPool;
use tracing::error;

use async_trait::async_trait;

#[async_trait]
pub trait ServiceFactory<T> {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<T>, anyhow::Error>;
}

pub struct EventServiceFactory;

pub struct EventService {
    repository: Arc<dyn Repository + Send + Sync>,
}

#[async_trait]
pub trait Service {
    async fn create_event(&self, dto: EventDTO) -> anyhow::Result<Response>;
}

impl EventService {
    pub fn new(repository: Arc<dyn Repository + Send + Sync>) -> EventService {
        EventService { repository }
    }
}

#[async_trait]
impl ServiceFactory<EventService> for EventServiceFactory {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<EventService>, anyhow::Error> {
        let repository: Arc<dyn Repository + Send + Sync> = Arc::new(EventRepository::new(pool));
        let service = Arc::new(EventService::new(repository));
        Ok(service)
    }
}

#[async_trait]
impl Service for EventService {
    async fn create_event(&self, dto: EventDTO) -> anyhow::Result<Response> {
        let event = Event::new(dto);

        match self.repository.save(event).await {
            Ok(_) => Ok(Response {
                status: Some(201),
                message: Some(String::from("Event created successfully")),
            }),
            Err(e) => {
                error!("Error creating event: {:?}", e);
                Ok(Response {
                    status: Some(500),
                    message: Some(String::from("Internal server error")),
                })
            }
        }
    }
}
