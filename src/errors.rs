use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Serialize)]
struct ErrorMessage {
    success: bool,
    message: String,
}

#[derive(Debug)]
pub struct InternalServerError;

impl warp::reject::Reject for InternalServerError {}

#[derive(Debug)]
pub struct RoomNotFoundError;

impl warp::reject::Reject for RoomNotFoundError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else if let Some(RoomNotFoundError) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = "Requested room does not exist";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else {
        // catching all other errors
        eprintln!("unhandled rejection: {:?}", err);

        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    }

    let json = warp::reply::json(&ErrorMessage {
        success: false,
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
