use async_trait::async_trait;
use sqlx::postgres::PgPool;
use sqlx::Row;

use crate::domain::event::Event;

pub struct EventRepositoryImpl {
    pool: PgPool,
}

#[async_trait]
pub trait EventRepository {
    async fn save(&self, event: Event) -> anyhow::Result<i64, sqlx::Error>;
}

impl EventRepositoryImpl {
    pub fn new(pool: PgPool) -> EventRepositoryImpl {
        EventRepositoryImpl { pool }
    }
}

#[async_trait]
impl EventRepository for EventRepositoryImpl {
    async fn save(&self, event: Event) -> anyhow::Result<i64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO events (name, description, location, start_date, end_date, organizer_id, sustainable_practices)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id"#
        )
        .bind(&event.name)
        .bind(&event.description)
        .bind(&event.location)
        .bind(event.start_date)
        .bind(event.end_date)
        .bind(event.organizer_id)
        .bind(&event.sustainable_practice)
        .fetch_one(&self.pool)
        .await?;

        let id: i64 = result.get::<i64, _>(0);
        Ok(id)
    }
}
