use std::sync::Arc;

use crate::domain::user::repository::{Repository, UserRepository};
use crate::domain::user::{User, UserDTO};

use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::Error;
use tracing::error;

use crate::domain::response::Response;

#[async_trait]
impl ServiceFactory<UserService> for UserServiceFactory {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<UserService>, anyhow::Error> {
        let repository: Arc<dyn Repository + Send + Sync> = Arc::new(UserRepository::new(pool));
        let service = Arc::new(UserService::new(repository));
        Ok(service)
    }
}

#[async_trait]
pub trait ServiceFactory<T> {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<T>, anyhow::Error>;
}

pub struct UserServiceFactory;

pub struct UserService {
    repository: Arc<dyn Repository + Send + Sync>,
}

#[async_trait]
pub trait Service {
    async fn create_user(&self, dto: UserDTO) -> anyhow::Result<Response>;
    async fn find_by_email(&self, email: String) -> anyhow::Result<UserDTO, sqlx::Error>;
}

impl UserService {
    pub fn new(repository: Arc<dyn Repository + Send + Sync>) -> UserService {
        UserService { repository }
    }
}

#[async_trait]
impl Service for UserService {
    async fn create_user(&self, dto: UserDTO) -> anyhow::Result<Response> {
        let user = User::new(dto);
        match self.find_by_email(user.email.clone()).await {
            Ok(_) => Ok(Response {
                status: Some(400),
                message: Some(String::from("Email already exists")),
            }),
            Err(Error::RowNotFound) => match self.repository.save(user).await {
                Ok(_) => Ok(Response {
                    status: Some(201),
                    message: Some(String::from("User created")),
                }),
                Err(e) => {
                    error!("Failed to create user: {:?}", e);
                    Ok(Response {
                        status: Some(500),
                        message: Some(String::from("Failed to create user")),
                    })
                }
            },
            Err(e) => {
                error!("Failed to find user by email: {:?}", e);
                Ok(Response {
                    status: Some(500),
                    message: Some(String::from("Failed to find user by email")),
                })
            }
        }
    }

    async fn find_by_email(&self, email: String) -> anyhow::Result<UserDTO, sqlx::Error> {
        let user = self.repository.find_by_email(email).await?;
        Ok(UserDTO {
            name: user.name,
            email: user.email,
            password: String::from(""),
            roles: user.roles,
        })
    }
}
