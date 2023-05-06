use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct Test {
    abcd: String,
    key2: String,
}

pub fn rooms_filter() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let rooms_base = warp::path("rooms");

    let all_rooms = rooms_base
        .and(warp::get())
        .and(warp::path::end())
        .map(|| "All rooms route");

    let single_room = rooms_base
        .and(warp::get())
        .and(warp::path::param())
        .and(warp::path::end())
        .map(|room_id: String| format!("Single room data {}", room_id));

    let new_room = rooms_base
        .and(warp::post())
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(|body: Test| warp::reply::json(&body));

    all_rooms.or(single_room).or(new_room)
}
