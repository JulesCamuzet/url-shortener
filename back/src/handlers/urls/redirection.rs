use axum::{extract::{Path, State}, http::StatusCode, response::Redirect};

use crate::{
    errors::HandlerError,
    modules::urls::roll_redirection::{
        roll_redirection_link, RollRedirectionLinkError
    },
    AppState
};

pub async fn handle_redirection(
    Path(short_value): Path<String>,
    State(state): State<AppState>
) -> Result<Redirect, HandlerError> {
    match roll_redirection_link(short_value, state.pool).await {
        Err(e) => match e {
            RollRedirectionLinkError::Unknown => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: "UNNKOWN".to_string(),
                message: "An unknown error has occured.".to_string()
            }),
            // TODO define default not found page
            RollRedirectionLinkError::NotFound => Ok(Redirect::to("https://yoyoyo.com"))
        },
        Ok(original_value) => Ok(Redirect::to(&original_value))
    }
}
