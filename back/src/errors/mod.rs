use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub struct HandlerError {
    pub status: StatusCode,
    pub code: String,
    pub message: String
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let response_body = format!("{{ message: {}, code: {} }}", self.message, self.code);
        (self.status, response_body).into_response()
    }
}
