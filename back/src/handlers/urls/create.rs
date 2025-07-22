use axum::{
    extract::{rejection::JsonRejection, State},
    http::{HeaderMap, StatusCode},
    Json
};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{
    errors::{json_rejection::get_handler_error_from_json_rejection, HandlerError},
    middlewares::auth::check_auth,
    models::user::User,
    modules::urls::create::{create_url, CreateUrlError, CreateUrlInput},
    AppState
};

#[derive(Deserialize)]
pub struct Payload {
    pub url: String
}

#[derive(Serialize)]
pub struct Output {
    pub id: i32,
    pub short_value: String,
    pub original_value: String
}

#[debug_handler]
pub async fn handle_create_url(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Result<Json<Payload>, JsonRejection>
) -> Result<Json<Output>, HandlerError> {
    let user: Option<User> = match check_auth(&headers, &state.pool).await {
        Err(_) => None,
        Ok(user) => Some(user)
    };
    
    let payload = match payload {
        Ok(Json(payload)) => payload,
        Err(rejection) =>  return Err(get_handler_error_from_json_rejection(rejection))
    };
    
    match create_url(CreateUrlInput {
        url: payload.url,
        user_id: match user {
            None => None,
            Some(user) => Some(user.id)
        },
        pool: state.pool
    }).await {
        Ok(output) => Ok(Json(Output {
            id: output.id,
            original_value: output.original_value,
            short_value: output.short_value
        })),
        Err(e) => match e {
            CreateUrlError::Unknown => Err(HandlerError {
                code: "UNKNOWN".to_string(),
                message: "An unknown error has occured.".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR
            }),
            CreateUrlError::WrongFormat => Err(HandlerError {
                code: "WRONG_FORMAT".to_string(),
                message: "The format of the url is wrong.".to_string(),
                status: StatusCode::BAD_REQUEST
            })
        },
    }
}
