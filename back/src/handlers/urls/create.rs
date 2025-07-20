use axum::{extract::{rejection::JsonRejection, State}, http::HeaderMap, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

use crate::{errors::{json_rejection::get_handler_error_from_json_rejection, HandlerError}, middlewares::auth::check_auth, models::user::User, AppState};

#[derive(Deserialize)]
pub struct Payload {
    pub url: String
}

#[derive(Serialize)]
pub struct Output {
    pub id: i32
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
    
    Ok(Json(Output { id: 3 }))
}
