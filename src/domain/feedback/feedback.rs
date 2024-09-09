use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
struct FeedBack {
    id: i32,
    event_id: i32,
    user_id: i32,
    rating: i32,
    comments: Vec<String>,
    created_at: NaiveDateTime,
}

impl FeedBack {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn event_id(&self) -> i32 {
        self.event_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn rating(&self) -> i32 {
        self.rating
    }

    pub fn comments(&self) -> Vec<String> {
        self.comments.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
