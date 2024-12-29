use serde::Deserialize;

#[derive(sqlx::Type, Debug, Clone, Copy, Deserialize)]
#[sqlx(type_name = "backend.role")]
pub enum Role {
    Organizer,
    Participant,
}
