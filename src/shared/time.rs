use actix_web::cookie::time::Month;
use chrono::{Datelike, NaiveDateTime, Timelike};

use sqlx::types::time::{Date, PrimitiveDateTime, Time};

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

pub fn convert_chrono_to_sqlx(naive: NaiveDateTime) -> PrimitiveDateTime {
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
