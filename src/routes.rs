use crate::{
    handlers::{
        create_new_room, fetch_active_rooms, fetch_available_rooms, fetch_occupancies,
        fetch_single_room,
    },
    with_db, DBPool,
};
use uuid::Uuid;
use warp::Filter;

pub fn rooms_routes(
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
