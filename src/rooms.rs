use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::Filter;

use crate::{
    errors::{InternalServerError, RoomNotFoundError},
    with_db, DBPool,
};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Room {
    id: Uuid,
    name: String,
    room_id: String,
    capacity: String,
    time_limit: String,
    link: String,
    comments: String,
}

#[derive(Serialize, Deserialize)]
struct NewRoom {
    name: String,
    room_id: String,
    capacity: String,
    time_limit: String,
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

    let single_room = rooms_base
        .and(warp::get())
        .and(warp::path::param::<Uuid>())
        .and(warp::path::end())
        .and(with_db(db_pool.clone()))
        .and_then(fetch_single_room);

    let new_room = rooms_base
        .and(warp::post())
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(with_db(db_pool.clone()))
        .map(|body: NewRoom, _db: DBPool| warp::reply::json(&body));

    all_rooms.or(single_room).or(new_room)
}

async fn fetch_available_rooms(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, Room>(
        "SELECT * FROM rooms, occupancies WHERE rooms.id != occupancies.occupied_room_id",
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
    let query_result = sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE rooms.id = $1")
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
