use axum::{extract::rejection::JsonRejection, Json};
use serde::{Deserialize, Serialize};

use crate::errors::{json_rejection::get_handler_error_from_json_rejection, HandlerError};

#[derive(Deserialize)]
pub struct Payload {
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct Output {
    success: bool
}

pub async fn handle_login(payload: Result<Json<Payload>, JsonRejection>) -> Result<Output, HandlerError> {
    let payload = match payload {
        Err(rejection) => return Err(get_handler_error_from_json_rejection(rejection)),
        Ok(Json(payload)) => payload
    };

    Ok(Output { success: true })
}
