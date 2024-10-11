use actix_web::cookie::time::Month;
use chrono::{Datelike, NaiveDateTime, Timelike, Utc};
use serde::Deserialize;
use sqlx::{
    types::time::{Date, PrimitiveDateTime, Time},
    FromRow,
};

use crate::domain::sustainable_practice::SustainablePractice;

#[derive(Debug, FromRow)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub start_date: PrimitiveDateTime,
    pub end_date: PrimitiveDateTime,
    pub organizer_id: i32,
    pub max_participants: i32,
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
    pub organizer_id: i32,
    pub max_participants: i32,
    pub sustainable_practice: Vec<SustainablePractice>,
}

pub mod parse_time {
    use chrono::naive::NaiveDateTime;
    use serde::de::Error as SerdeError;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(dt: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        dt.format("%Y-%m-%d %H:%M:%S%.f")
            .to_string()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let t = String::deserialize(deserializer)?;
        // it doesn't try to handle the error, just unwraps
        NaiveDateTime::parse_from_str(&t, "%Y-%m-%d %H:%M:%S%.f")
            .map_err(|e| D::Error::custom(format!("Error when parse datetime: {}", e)))
    }
}

fn convert_chrono_to_sqlx(naive: NaiveDateTime) -> PrimitiveDateTime {
    // Extrai o mês como enum Month do sqlx::types::time
    let month_enum = match naive.month() {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Mês inválido!"),
    };

    // Converte para sqlx::types::time::Date
    let date = Date::from_calendar_date(naive.year(), month_enum, naive.day().try_into().unwrap())
        .unwrap();

    // Converte para sqlx::types::time::Time
    let time = Time::from_hms(
        naive.hour().try_into().unwrap(),
        naive.minute().try_into().unwrap(),
        naive.second().try_into().unwrap(),
    )
    .unwrap();

    // Cria o PrimitiveDateTime
    PrimitiveDateTime::new(date, time)
}
