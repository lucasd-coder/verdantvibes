use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use sqlx::{types::time::PrimitiveDateTime, FromRow};

use crate::domain::sustainable_practice::SustainablePractice;
use crate::shared::time::{convert_chrono_to_sqlx, parse_time};

#[derive(Debug, FromRow)]
pub struct Event {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub location: String,
    pub start_date: PrimitiveDateTime,
    pub end_date: PrimitiveDateTime,
    pub organizer_id: i64,
    pub max_participants: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub sustainable_practice: Vec<SustainablePractice>,
}

impl Event {
    pub fn new(dto: EventDTO) -> Event {
        let now = Utc::now();
        Event {
            id: 0,
            name: dto.name,
            description: dto.description,
            location: dto.location,
            start_date: convert_chrono_to_sqlx(dto.start_date),
            end_date: convert_chrono_to_sqlx(dto.end_date),
            organizer_id: dto.organizer_id,
            max_participants: dto.max_participants,
            sustainable_practice: dto.sustainable_practice,
            updated_at: convert_chrono_to_sqlx(NaiveDateTime::default()),
            created_at: convert_chrono_to_sqlx(now.naive_utc()),
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct EventDTO {
    pub name: String,
    pub description: String,
    pub location: String,
    #[serde(with = "parse_time")]
    pub start_date: NaiveDateTime,
    #[serde(with = "parse_time")]
    pub end_date: NaiveDateTime,
    pub organizer_id: i64,
    pub max_participants: i64,
    pub sustainable_practice: Vec<SustainablePractice>,
}
