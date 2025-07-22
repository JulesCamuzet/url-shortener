use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};

use crate::{
    errors::HandlerError,
    modules::urls::get_original_value::{
        get_url_original_value,
        GetUrlOriginalValueError,
        GetUrlOriginalValueInput
    },
    AppState
};

pub async fn handle_redirection(
    Path(short_value): Path<String>,
    State(state): State<AppState>
) -> Result<Redirect, HandlerError> {
    match get_url_original_value(GetUrlOriginalValueInput {
        short_value,
        pool: state.pool
    }).await {
        Err(e) => match e {
            GetUrlOriginalValueError::Unknown => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: "UNNKOWN".to_string(),
                message: "An unknown error has occured.".to_string()
            }),
            // TODO define default not found page
            GetUrlOriginalValueError::NotFound => Ok(Redirect::to("https://yoyoyo.com"))
        },
        Ok(original_value) => Ok(Redirect::to(&original_value))
    }
}
