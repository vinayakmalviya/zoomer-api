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

#[derive(Debug)]
pub struct RoomWithNameExistsError;

impl warp::reject::Reject for RoomWithNameExistsError {}

#[derive(Debug)]
pub struct RoomWithIdExistsError;

impl warp::reject::Reject for RoomWithIdExistsError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else if let Some(RoomNotFoundError) = err.find() {
        code = StatusCode::NOT_FOUND;
        message = "Requested room does not exist";
    } else if let Some(RoomWithNameExistsError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Room with same name exists";
    } else if let Some(RoomWithIdExistsError) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "Room with same room id exists";
    } else if let Some(InternalServerError) = err.find() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid request payload, check if all fields are sent";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::NOT_FOUND;
        message = "Not found";
    } else {
        // catching all other errors
        eprintln!("unhandled rejection: {:?}", err);

        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
    }

    let json = warp::reply::json(&ErrorMessage {
        success: false,
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
