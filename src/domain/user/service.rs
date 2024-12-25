use std::sync::Arc;

use crate::domain::user::repository::{UserRepository, UserRepositoryImpl};
use crate::domain::user::{User, UserDTO};

use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::Error;
use tracing::error;

use crate::domain::response::Response;

#[async_trait]
impl ServiceFactory<UserServiceImpl> for UserServiceFactory {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<UserServiceImpl>, anyhow::Error> {
        let repository: Arc<dyn UserRepository + Send + Sync> = Arc::new(UserRepositoryImpl::new(pool));
        let service = Arc::new(UserServiceImpl::new(repository));
        Ok(service)
    }
}

#[async_trait]
pub trait ServiceFactory<T> {
    async fn create_service(&self, pool: PgPool) -> Result<Arc<T>, anyhow::Error>;
}

pub struct UserServiceFactory;

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

#[async_trait]
pub trait UserService {
    async fn create_user(&self, dto: UserDTO) -> anyhow::Result<Response>;
    async fn find_by_email(&self, email: String) -> anyhow::Result<UserDTO, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<UserDTO, sqlx::Error>;
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> UserServiceImpl {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
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

    async fn find_by_id(&self, id: i32) -> anyhow::Result<UserDTO, sqlx::Error> {
        let user = self.repository.find_by_id(id).await?;
        Ok(UserDTO {
            name: user.name,
            email: user.email,
            password: String::from(""),
            roles: user.roles,
        })
    }
}
