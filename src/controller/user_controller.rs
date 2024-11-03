use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;
use tracing::error;

use crate::domain::user::service::{Service, ServiceFactory, UserServiceFactory};
use crate::{controller::APIResult, domain::user::UserDTO};

#[actix_web::post("/users")]
pub async fn save(pool: web::Data<PgPool>, payload: web::Json<UserDTO>) -> APIResult {
    let dto = UserDTO {
        email: payload.email.clone(),
        name: payload.name.clone(),
        password: payload.password.clone(),
        roles: payload.roles.clone(),
    };
    let factory = UserServiceFactory;

    let user_service = factory.create_service(pool.get_ref().clone()).await?;
    match user_service.create_user(dto).await {
        Ok(r) => Ok(r.http_response()),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
