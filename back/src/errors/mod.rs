use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

pub mod json_rejection;

pub struct HandlerError {
    pub status: StatusCode,
    pub code: String,
    pub message: String
}

#[derive(Serialize)]
struct ErrorResponseBody {
    code: String,
    message: String
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let response_body = Json(ErrorResponseBody {
            code: self.code,
            message: self.message
        });

        (self.status, response_body).into_response()
    }
}
