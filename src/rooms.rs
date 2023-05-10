use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::Filter;

use crate::{
    errors::{
        InternalServerError, RoomNotFoundError, RoomWithIdExistsError, RoomWithNameExistsError,
    },
    with_db, DBPool,
};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
struct Room {
    id: Uuid,
    name: String,
    room_id: String,
    capacity: String,
    time_limit: String,
    link: String,
    comments: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
struct Occupancy {
    id: i32,
    occupied_room_id: Uuid,
    #[serde(with = "my_date_format")]
    occupied_until: DateTime<Utc>,
    meeting_title: String,
    comments: String,
}

#[derive(Deserialize)]
struct NewRoom {
    name: String,
    room_id: String,
    capacity: String,
    time_limit: u64,
    link: String,
    comments: String,
}

pub fn rooms_filter(
    db_pool: DBPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let rooms_base = warp::path("rooms");

    let all_rooms = rooms_base
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_available_rooms);

    let available_rooms = rooms_base
        .and(warp::get())
        .and(warp::path("available"))
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_available_rooms);

    let active_rooms = rooms_base
        .and(warp::get())
        .and(warp::path("active"))
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_active_rooms);

    let single_room = rooms_base
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_single_room);

    let occupancies = rooms_base
        .and(warp::get())
        .and(warp::path("occupancies"))
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_occupancies);

    let new_room = rooms_base
        .and(warp::post())
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .and_then(create_new_room);

    all_rooms
        .or(available_rooms)
        .or(active_rooms)
        .or(single_room)
        .or(occupancies)
        .or(new_room)
}

async fn fetch_available_rooms(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, Room>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments 
        FROM 
          rooms 
          LEFT JOIN occupancies ON rooms.id != occupancies.occupied_room_id",
    )
    .fetch_all(&db)
    .await;

    match query_result {
        Ok(rooms) => {
            let resp = json!({
                "rooms": rooms,
            });

            Ok(warp::reply::json(&resp))
        }
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
}

async fn fetch_active_rooms(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, Room>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments 
        FROM 
          rooms 
          JOIN occupancies ON rooms.id = occupancies.occupied_room_id",
    )
    .fetch_all(&db)
    .await;

    match query_result {
        Ok(rooms) => {
            let resp = json!({
                "rooms": rooms,
            });

            Ok(warp::reply::json(&resp))
        }
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
}

async fn fetch_single_room(room_id: Uuid, db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, Room>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments 
        FROM 
          rooms 
        WHERE 
          id = $1",
    )
    .bind(room_id)
    .fetch_one(&db)
    .await;

    match query_result {
        Ok(room) => {
            let resp = json!({ "room_details": room });

            Ok(warp::reply::json(&resp))
        }
        Err(sqlx::Error::RowNotFound) => Err(warp::reject::custom(RoomNotFoundError)),
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
}

async fn fetch_occupancies(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, Occupancy>("SELECT * FROM occupancies")
        .fetch_all(&db)
        .await;

    match query_result {
        Ok(occupancies) => {
            let resp = json!({
                "occupancies": occupancies,
            });

            Ok(warp::reply::json(&resp))
        }
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
}

async fn create_new_room(
    room_data: NewRoom,
    db: DBPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let check_query = sqlx::query_as::<_, Room>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments 
        FROM 
          rooms 
        WHERE 
          name = $1
          OR room_id = $2",
    )
    .bind(&room_data.name)
    .bind(&room_data.room_id)
    .fetch_all(&db)
    .await;

    match check_query {
        Ok(res) => {
            if res.len() > 0 {
                let found_room = &res[0];

                if found_room.name == room_data.name {
                    return Err(warp::reject::custom(RoomWithNameExistsError));
                } else if found_room.room_id == room_data.room_id {
                    return Err(warp::reject::custom(RoomWithIdExistsError));
                }
            }
        }
        Err(err) => {
            dbg!(err);

            return Err(warp::reject::custom(InternalServerError));
        }
    }

    let interval = Duration::from_secs(room_data.time_limit * 60);

    let query_result = sqlx::query_as::<_, Room>(
        "INSERT INTO rooms(
          name, room_id, capacity, time_limit, 
          link, comments
        ) 
        VALUES 
          ($1, $2, $3, $4 :: interval, $5, $6) RETURNING id, 
          name, 
          room_id, 
          capacity, 
          TO_CHAR(time_limit, 'HH24:MI:SS') as time_limit, 
          link, 
          comments",
    )
    .bind(room_data.name)
    .bind(room_data.room_id)
    .bind(room_data.capacity)
    .bind(interval)
    .bind(room_data.link)
    .bind(room_data.comments)
    .fetch_all(&db)
    .await;

    match query_result {
        Ok(room) => {
            let resp = json!({ "room_details": room[0] });

            Ok(warp::reply::json(&resp))
        }
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
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
