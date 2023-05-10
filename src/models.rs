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

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Occupancy {
    pub id: i32,
    pub occupied_room_id: Uuid,
    #[serde(with = "my_date_format")]
    pub occupied_until: DateTime<Utc>,
    pub meeting_title: String,
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

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
