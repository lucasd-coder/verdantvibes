use crate::domain::{user::User, user::UserDTO};
use serde_json::value::RawValue as JsonRawValue;
use sqlx::postgres::PgPool;
use sqlx::Error;
use sqlx::Row;
use tracing::error;

use crate::domain::response::Response;

pub async fn create_user(pool: &PgPool, dto: UserDTO) -> anyhow::Result<Response> {
    match find_by_email(pool, dto.email.clone()).await {
        Ok(_) => Ok(Response {
            status: Some(400),
            message: Some(String::from("Email already exists")),
        }),
        Err(Error::RowNotFound) => save_user(pool, dto).await,
        Err(e) => {
            error!("Failed to find user by email: {:?}", e);
            Ok(Response {
                status: Some(500),
                message: Some(String::from("Failed to find user by email")),
            })
        }
    }
}

pub async fn find_by_email(pool: &PgPool, email: String) -> anyhow::Result<UserDTO, sqlx::Error> {
    let rec = sqlx::query(
        r#"
    SELECT name, email, roles FROM users
    WHERE email = $1"#,
    )
    .bind(&email)
    .fetch_one(pool)
    .await?;

    let value: &JsonRawValue = rec.try_get(0)?;

    let dto = serde_json::from_str(value.get()).map_err(|e| {
        error!("Failed to deserialize user data: {}", e);
        sqlx::Error::ColumnDecode {
            index: "json decode".to_string(),
            source: Box::new(e),
        }
    })?;

    Ok(dto)
}

pub async fn save_user(pool: &PgPool, dto: UserDTO) -> anyhow::Result<Response> {
    let user = User::new(dto);

    sqlx::query(
        r#"
      INSERT INTO users (name, email, password_hash, roles, created_at)
      VALUES ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.roles)
    .bind(user.created_at)
    .execute(pool)
    .await?;

    Ok(Response {
        status: Some(201),
        message: Some(String::from("User created")),
    })
}
