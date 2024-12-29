use crate::domain::role::Role;
use crate::domain::user::User;
use crate::shared::time::convert_chrono_to_sqlx;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use sqlx::Row;

pub struct UserRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: String) -> anyhow::Result<User, sqlx::Error>;
    async fn save(&self, user: User) -> anyhow::Result<i64, sqlx::Error>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<User, sqlx::Error>;
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: String) -> anyhow::Result<User, sqlx::Error> {
        let row: (String, String, Vec<Role>) = sqlx::query_as(
            r#"
              SELECT name, email, roles FROM backend.users
              WHERE email = $1"#,
        )
        .bind(&email)
        .fetch_one(&self.pool)
        .await?;
        let user = User {
            id: 0,
            name: row.0,
            email: row.1,
            password_hash: String::from(""),
            roles: row.2,
            updated_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
            created_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
        };
        Ok(user)
    }

    async fn save(&self, user: User) -> anyhow::Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
                INSERT INTO backend.users (name, email, password_hash, roles, created_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id"#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.roles)
        .bind(user.created_at)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = result.get::<i64, _>(0);
        Ok(id)
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<User, sqlx::Error> {
        let row: (String, String, Vec<Role>) = sqlx::query_as(
            r#"
              SELECT name, email, roles FROM backend.users
              WHERE id = $1"#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        let user = User {
            id,
            name: row.0,
            email: row.1,
            password_hash: String::from(""),
            roles: row.2,
            updated_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
            created_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
        };
        Ok(user)
    }
}
