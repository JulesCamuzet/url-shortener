use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::{errors::HandlerError, models::user::User};

#[derive(Deserialize)]
pub struct Payload {
    pub email: String,
    pub password: String
}

#[debug_handler]
pub async fn handle_register(payload: Result<Json<Payload>, JsonRejection>) -> Result<Json<User>, HandlerError> {
    match payload {
        Ok(payload) => {
            let user = User {
                id: 0,
                email: "jcamuzet@yahoo.com".to_string(),
                is_verified: true,
                created_at: "today".to_string(),
                password: "opzj".to_string(),
                reset_password_token: None,
                verification_token: "pzofk".to_string()
            };
            Ok(Json(user))
        },
        Err(JsonRejection::JsonDataError(_)) => {
            Err(HandlerError {
                message: "Wrong payload".to_string(),
                code: "WRONG_PAYLOAD".to_string(),
                status: StatusCode::BAD_REQUEST
            })
        },
        _ => {
            Err(HandlerError {
                message: "An unknown error has occured".to_string(),
                code: "UNKNOWN".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR
            })
        }
    }
}
