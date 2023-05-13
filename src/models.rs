use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub room_id: String,
    pub capacity: String,
    pub time_limit: String,
    pub link: String,
    pub comments: String,
}

#[derive(Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub room_id: String,
    pub capacity: String,
    pub time_limit: u64,
    pub link: String,
    pub comments: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct ActiveRoom {
    pub id: Uuid,
    pub name: String,
    pub room_id: String,
    pub capacity: String,
    pub time_limit: String,
    pub link: String,
    pub comments: String,
    pub is_active: bool,
    pub occupied_until: DateTime<Utc>,
    pub meeting_title: String,
    pub meeting_comments: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Occupancy {
    pub id: i32,
    pub occupied_room_id: Uuid,
    pub occupied_until: DateTime<Utc>,
    pub meeting_title: String,
    pub comments: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct NewOccupancy {
    pub occupied_room_id: Uuid,
    pub occupied_until: DateTime<Utc>,
    pub meeting_title: String,
    pub comments: String,
}
