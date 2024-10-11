use crate::domain::{event::Event, event::EventDTO};
use sqlx::postgres::PgPool;

pub async fn create_event(pool: &PgPool, dto: EventDTO) -> anyhow::Result<()> {
    let event = Event::new(dto);

    sqlx::query(
            r#"
            INSERT INTO events (name, description, location, start_date, end_date, organizer_id, sustainable_practices)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(&event.name)
        .bind(&event.description)
        .bind(&event.location)
        .bind(event.start_date)
        .bind(event.end_date)
        .bind(event.organizer_id)
        .bind(&event.sustainable_practice) // Aqui estamos passando o array de enums
        .fetch_one(pool)
        .await?;

    Ok(())
}
