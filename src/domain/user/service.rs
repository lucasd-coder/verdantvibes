use crate::domain::{user::User, user::UserDTO};
use anyhow::Ok;
use sqlx::postgres::PgPool;

pub async fn create_user(pool: &PgPool, dto: UserDTO) -> anyhow::Result<()> {
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

    Ok(())
}
