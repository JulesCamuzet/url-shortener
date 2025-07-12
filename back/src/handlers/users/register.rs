use axum::{extract::{rejection::JsonRejection, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    errors::{json_rejection::get_handler_error_from_json_rejection, HandlerError},
    modules::users::create::{create_user, CreateUserError, CreateUserInput}, AppState
};

#[derive(Deserialize)]
pub struct Payload {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct Output {
    id: i32
}

pub async fn handle_register(State(state): State<AppState>, payload: Result<Json<Payload>, JsonRejection>) -> Result<Json<Output>, HandlerError> {
    let payload = match payload {
        Err(rejection) => return Err(get_handler_error_from_json_rejection(rejection)),
        Ok(Json(payload)) => payload
    };

    match create_user(CreateUserInput {
        email: payload.email,
        password: payload.password,
        pool: state.pool
    }).await {
        Ok(output) => Ok(Json(Output { id: output.id })),
        Err(e) => match e {
            CreateUserError::EmailAlreadyExists => Err(HandlerError {
                code: "EMAIL_ALREADY_EXISTS".to_string(),
                message: "This email already exists.".to_string(),
                status: StatusCode::CONFLICT
            }),
            CreateUserError::InvalidEmailFormat => Err(HandlerError {
                code: "INVALID_EMAIL_FORMAT".to_string(),
                message: "The format of the email is invalid.".to_string(),
                status: StatusCode::BAD_REQUEST
            }),
            CreateUserError::InvalidPasswordFormat => Err(HandlerError {
                code: "INVALID_PASSWORD_FORMAT".to_string(),
                message: "The format of the password is invalid.".to_string(),
                status: StatusCode::BAD_REQUEST
            }),
            CreateUserError::Unknown => Err(HandlerError {
                code: "UNKNOWN".to_string(),
                message: "An unknown error has occured.".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR
            })
        }
    }
}
