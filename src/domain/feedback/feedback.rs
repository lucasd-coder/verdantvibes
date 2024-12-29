use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
struct FeedBack {
    id: i64,
    event_id: i64,
    user_id: i64,
    rating: i64,
    comments: Vec<String>,
    created_at: NaiveDateTime,
}

impl FeedBack {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn event_id(&self) -> i64 {
        self.event_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn rating(&self) -> i64 {
        self.rating
    }

    pub fn comments(&self) -> Vec<String> {
        self.comments.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
