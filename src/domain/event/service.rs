use std::sync::Arc;

use crate::domain::event::repository::{EventRepositoryImpl, EventRepository};
use crate::domain::response::Response;
use crate::domain::user;
use crate::domain::user::repository::UserRepository;
use crate::domain::user::service::{UserService, UserServiceImpl};
use crate::domain::{event::Event, event::EventDTO};

use sqlx::postgres::PgPool;
use tracing::error;

use async_trait::async_trait;

#[async_trait]
pub trait ServiceFactory<T> {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<T>, anyhow::Error>;
}

pub struct EventServiceFactory;

pub struct EventServiceImpl {
    repository: Arc<dyn EventRepository + Send + Sync>,
    user_service: Arc<dyn UserService + Send + Sync>,
}

#[async_trait]
pub trait EventService {
    async fn create_event(&self, dto: EventDTO) -> anyhow::Result<Response>;
}

impl EventServiceImpl {
    pub fn new(
        repository: Arc<dyn EventRepository + Send + Sync>,
        user_service: Arc<dyn UserService + Send + Sync>,
    ) -> EventServiceImpl {
        EventServiceImpl { repository, user_service }
    }
}

#[async_trait]
impl ServiceFactory<EventServiceImpl> for EventServiceFactory {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<EventServiceImpl>, anyhow::Error> {
        let event_repository: Arc<dyn EventRepository + Send + Sync> = Arc::new(EventRepositoryImpl::new(pool.clone()));
        let user_repository: Arc<dyn UserRepository + Send + Sync> = Arc::new(user::repository::UserRepositoryImpl::new(pool.clone()));
        let user_service = Arc::new(UserServiceImpl::new(user_repository));
        let event_service = Arc::new(EventServiceImpl::new(event_repository, user_service));
        Ok(event_service)
    }
}

#[async_trait]
impl EventService for EventServiceImpl {
    async fn create_event(&self, dto: EventDTO) -> anyhow::Result<Response> {
        let event = Event::new(dto);
        match self.user_service.find_by_id(event.organizer_id).await {
            Ok(_) => {
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
            Err(_) => {
                return Ok(Response {
                    status: Some(404),
                    message: Some(String::from("User not found")),
                });
            }
        }

    }
}
