use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;
use tracing::error;

use crate::domain::user::service::create_user;
use crate::{controller::APIResult, domain::user::UserDTO};

#[actix_web::post("/users")]
pub async fn save(pool: web::Data<PgPool>, payload: web::Json<UserDTO>) -> APIResult {
    let dto = UserDTO {
        email: payload.email.clone(),
        name: payload.name.clone(),
        password: payload.password.clone(),
        roles: payload.roles.clone(),
    };

    match create_user(&pool, dto).await {
        Ok(r) => Ok(r.http_response()),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            Ok(HttpResponse::InternalServerError().finish())
        }
    }
}
