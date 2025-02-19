use serde::Serialize;
use warp::{http::StatusCode, reject::Reject, Rejection, Reply};
use warp::filters::body::BodyDeserializeError;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
}

impl Reject for ErrorMessage {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(e) = err.find::<BodyDeserializeError>() {
        let error_message = json!({
            "status": "fail",
            "error": format!("Request body deserialize error: {}", e)
        });

        return Ok(warp::reply::with_status(warp::reply::json(&error_message), StatusCode::BAD_REQUEST));
    }

    if let Some(e) = err.find::<ErrorMessage>() {
        let error_message = json!({
            "status": "fail",
            "error": e.message
        });

        return Ok(warp::reply::with_status(warp::reply::json(&error_message), StatusCode::BAD_REQUEST));
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&json!({ "status": "fail", "error": "Unhandled error" })),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}
