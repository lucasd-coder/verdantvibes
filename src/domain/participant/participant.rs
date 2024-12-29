use sqlx::FromRow;

use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
struct Participant {
    id: i64,
    event_id: i64,
    user_id: i64,
    ticket_type: String,
    checked_in: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Participant {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn event_id(&self) -> i64 {
        self.event_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn ticket_type(&self) -> String {
        self.ticket_type
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
