use chrono::{NaiveDateTime, Utc};

use serde::Deserialize;
use sqlx::{types::time::PrimitiveDateTime, FromRow};

use crate::domain::role::Role;
use crate::shared::time::convert_chrono_to_sqlx;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub roles: Vec<Role>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl User {
    pub fn new(dto: UserDTO) -> User {
        let now = Utc::now();
        User {
            id: 0,
            name: dto.name,
            email: dto.email,
            password_hash: dto.password,
            roles: dto.roles,
            updated_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
            created_at: convert_chrono_to_sqlx(now.naive_utc()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<Role>,
}
