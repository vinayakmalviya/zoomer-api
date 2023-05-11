use std::time::Duration;

use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::{
        InternalServerError, RoomNotFoundError, RoomOccupiedError, RoomWithIdExistsError,
        RoomWithNameExistsError,
    },
    models::{ActiveRoom, NewOccupancy, NewRoom, Occupancy, Room},
    DBPool,
};

pub async fn fetch_current_state(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let available_rooms_query = sqlx::query_as::<_, Room>(
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
          LEFT OUTER JOIN occupancies ON rooms.id = occupancies.occupied_room_id 
        WHERE 
          occupancies.id IS NULL",
    )
    .fetch_all(&db)
    .await;

    let active_rooms_query = sqlx::query_as::<_, ActiveRoom>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments,
          TRUE as is_active,
          occupancies.occupied_until,
          occupancies.meeting_title,
          occupancies.comments as meeting_comments
        FROM 
          rooms 
          JOIN occupancies ON rooms.id = occupancies.occupied_room_id",
    )
    .fetch_all(&db)
    .await;

    match (available_rooms_query, active_rooms_query) {
        (Ok(available_rooms), Ok(active_rooms)) => {
            let resp = json!({
                "available_rooms": available_rooms,
                "active_rooms": active_rooms
            });

            Ok(warp::reply::json(&resp))
        }
        _ => Err(warp::reject::custom(InternalServerError)),
    }
}

pub async fn fetch_available_rooms(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
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
          LEFT OUTER JOIN occupancies ON rooms.id = occupancies.occupied_room_id 
        WHERE 
          occupancies.id IS NULL",
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

pub async fn fetch_active_rooms(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
    let query_result = sqlx::query_as::<_, ActiveRoom>(
        "SELECT 
          rooms.id, 
          rooms.name, 
          rooms.room_id, 
          rooms.capacity, 
          rooms.link, 
          TO_CHAR(rooms.time_limit, 'HH24:MI:SS') as time_limit, 
          rooms.comments,
          TRUE as is_active,
          occupancies.occupied_until,
          occupancies.meeting_title,
          occupancies.comments as meeting_comments
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

pub async fn fetch_single_room(
    room_id: Uuid,
    db: DBPool,
) -> Result<impl warp::Reply, warp::Rejection> {
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

pub async fn fetch_occupancies(db: DBPool) -> Result<impl warp::Reply, warp::Rejection> {
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

pub async fn create_new_room(
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

pub async fn handle_occupy_room(
    occupy_data: NewOccupancy,
    db: DBPool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let check_query = sqlx::query(
        "SELECT 
          occupied_room_id 
        FROM 
          occupancies
        WHERE 
          occupied_room_id = $1",
    )
    .bind(&occupy_data.occupied_room_id)
    .fetch_one(&db)
    .await;

    match check_query {
        Ok(_) => return Err(warp::reject::custom(RoomOccupiedError)),
        Err(sqlx::Error::RowNotFound) => (),
        Err(err) => {
            dbg!(err);

            return Err(warp::reject::custom(InternalServerError));
        }
    }

    let room_check_query = sqlx::query_as::<_, Room>(
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
    .bind(&occupy_data.occupied_room_id)
    .fetch_one(&db)
    .await;

    let room_data = match room_check_query {
        Ok(res) => res,
        Err(sqlx::Error::RowNotFound) => return Err(warp::reject::custom(RoomNotFoundError)),
        Err(err) => {
            dbg!(err);

            return Err(warp::reject::custom(InternalServerError));
        }
    };

    let insert_query = sqlx::query_as::<_, Occupancy>(
        "INSERT INTO occupancies(
          occupied_room_id, occupied_until, 
          meeting_title, comments
        ) 
        VALUES 
          ($1, $2, $3, $4) RETURNING id, 
          occupied_room_id, 
          occupied_until, 
          meeting_title, 
          comments",
    )
    .bind(&occupy_data.occupied_room_id)
    .bind(&occupy_data.occupied_until)
    .bind(&occupy_data.meeting_title)
    .bind(&occupy_data.comments)
    .fetch_all(&db)
    .await;

    match insert_query {
        Ok(occupancy) => {
            let active_room = ActiveRoom {
                id: room_data.id,
                name: room_data.name,
                room_id: room_data.room_id,
                capacity: room_data.capacity,
                time_limit: room_data.time_limit,
                link: room_data.link,
                comments: room_data.comments,
                is_active: true,
                occupied_until: occupancy[0].occupied_until.clone(),
                meeting_title: occupancy[0].meeting_title.clone(),
                meeting_comments: occupancy[0].comments.clone(),
            };

            let resp = json!({
                "room_details": active_room,
            });
            Ok(warp::reply::json(&resp))
        }
        Err(e) => {
            dbg!(e);

            Err(warp::reject::custom(InternalServerError))
        }
    }
}
