mod sustainable_practice;

use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct Event {
    id: i32,
    name: String,
    description: String,
    location: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    organizer_id: i32,
    max_participants: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    sustainable_practice: SustainablePractice,
}

impl Event {
    pub fn new(dto: EventDTO) -> Event {
        Event {
            dto.id,
            dto.name,
            dto.description,
            dto.location,
            dto.start_date,
            dto.end_date,
            dto.organizer_id,
            dto.max_participants,
            dto.sustainable_practice,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name
    }

    pub fn description(&self) -> String {
        self.description
    }

    pub fn location(&self) -> String {
        self.location
    }

    pub fn start_date(&self) -> NaiveDateTime {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDateTime {
        self.end_date
    }

    pub fn organizer_id(&self) -> i32 {
        self.organizer_id
    }

    pub fn max_participants(&self) -> i32 {
        self.max_participants
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    pub fn sustainable_practice(&self) -> SustainablePractice {
        self.sustainable_practice
    }
}

struct EventDTO {
    id: i32,
    name: String,
    description: String,
    location: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    organizer_id: i32,
    max_participants: i32,
    sustainable_practice: SustainablePractice,
}

impl EventDTO {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> String {
        self.name
    }

    pub fn description(&self) -> String {
        self.description
    }

    pub fn location(&self) -> String {
        self.location
    }

    pub fn start_date(&self) -> NaiveDateTime {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDateTime {
        self.end_date
    }

    pub fn organizer_id(&self) -> i32 {
        self.organizer_id
    }

    pub fn max_participants(&self) -> i32 {
        self.max_participants
    }

    pub fn sustainable_practice(&self) -> SustainablePractice {
        self.sustainable_practice
    }
}
