use axum::{
    Json,
    body::Body,
    extract::{State, rejection::JsonRejection},
    http::{StatusCode, header::SET_COOKIE},
    response::{AppendHeaders, IntoResponse, Response},
};
use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    errors::{HandlerError, json_rejection::get_handler_error_from_json_rejection},
    modules::users::authenticate::{
        AuthenticateUserError, AuthenticateUserInput, authenticate_user,
    },
};

#[derive(Deserialize)]
pub struct Payload {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Output {
    success: bool,
}

pub async fn handle_login(
    State(state): State<AppState>,
    payload: Result<Json<Payload>, JsonRejection>,
) -> Result<Response<Body>, HandlerError> {
    let payload = match payload {
        Err(rejection) => return Err(get_handler_error_from_json_rejection(rejection)),
        Ok(Json(payload)) => payload,
    };

    let authentication_result = authenticate_user(AuthenticateUserInput {
        email: payload.email,
        password: payload.password,
        pool: state.pool,
        private_key: state.private_key,
    })
    .await;

    let authentication_output = match authentication_result {
        Ok(output) => output,
        Err(e) => match e {
            AuthenticateUserError::InvalidCredentials => {
                return Err(HandlerError {
                    status: StatusCode::UNAUTHORIZED,
                    code: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid credentials.".to_string(),
                });
            }
            AuthenticateUserError::Unknown => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    code: "UNKNOWN".to_string(),
                    message: "An unknown error has occured.".to_string(),
                });
            }
        },
    };

    let cookie_expiration_date = chrono::offset::Local::now() + Duration::minutes(30);
    let cookie_expiration_string = cookie_expiration_date.to_rfc2822();

    let headers = AppendHeaders([(
        SET_COOKIE,
        format!(
            "auth={}; Expires={}",
            authentication_output.jwt, cookie_expiration_string
        ),
    )]);

    let response = (
        StatusCode::ACCEPTED,
        headers,
        Json(Output { success: true }),
    )
        .into_response();

    Ok(response)
}
