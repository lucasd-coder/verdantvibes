mod role;

use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
    role: Role,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl User {
    pub fn new(dto: UserDTO) -> User {
        User{
            dto.id,
            dto.name,
            dto.email,
            dto.password_hash,
            dto.role,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn password_hash(&self) -> String {
        self.password_hash.clone()
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(Debug)]
pub struct UserDTO {
    id: i32,
    name: String,
    email: String,
    password_hash: String,
    role: Role,
}

impl UserDTO {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn password_hash(&self) -> String {
        self.password_hash.clone()
    }

    pub fn role(&self) -> Role {
        self.role
    }
}
