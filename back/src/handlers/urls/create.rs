use axum::{
    extract::{rejection::JsonRejection, State},
    http::{HeaderMap, StatusCode},
    Json
};
use axum_macros::debug_handler;
use serde::{Serialize};

use crate::{
    dtos::url::CreateUrlDto,
    errors::{json_rejection::get_handler_error_from_json_rejection, HandlerError},
    middlewares::auth::check_auth,
    modules::urls::create::{create_url, CreateUrlError},
    AppState
};

#[derive(Serialize)]
pub struct Output {
    pub id: i32,
    pub short_value: String,
    pub redirections_ids: Vec<i32>
}

#[debug_handler]
pub async fn handle_create_url(
    State(state): State<AppState>,
    headers: HeaderMap,
    payload: Result<Json<CreateUrlDto>, JsonRejection>
) -> Result<Json<Output>, HandlerError> {
    let mut create_url_dto = match payload {
        Ok(Json(payload)) => payload,
        Err(rejection) =>  return Err(get_handler_error_from_json_rejection(rejection))
    };
    
    match check_auth(&headers, &state.pool).await {
        Err(_) => {},
        Ok(user) => create_url_dto.user_id = Some(user.id)
    };
    
    match create_url(create_url_dto, state.pool).await {
        Ok(output) => Ok(Json(Output {
            id: output.id,
            short_value: output.short_value,
            redirections_ids: output.redirections_ids
        })),
        Err(e) => match e {
            CreateUrlError::Unknown => Err(HandlerError {
                code: "UNKNOWN".to_string(),
                message: "An unknown error has occured.".to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR
            }),
            CreateUrlError::WrongProbabilityScore => Err(HandlerError {
               code: "WRONG_PROBABILITY_SCORE".to_string(),
               message: "The probability score of the redirection is too big or too small.".to_string(),
               status: StatusCode::BAD_REQUEST
            }),
            CreateUrlError::WrongFormat => Err(HandlerError {
                code: "WRONG_FORMAT".to_string(),
                message: "The format of the url is wrong.".to_string(),
                status: StatusCode::BAD_REQUEST
            })
        },
    }
}
