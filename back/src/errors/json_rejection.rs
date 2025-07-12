use axum::{extract::rejection::JsonRejection, http::StatusCode};

use crate::errors::HandlerError;

pub fn get_handler_error_from_json_rejection(rejection: JsonRejection) -> HandlerError {
    match rejection {
        JsonRejection::JsonDataError(_) => HandlerError {
            status: StatusCode::BAD_REQUEST,
            code: "WRONG_PAYLOAD".to_string(),
            message: "The payload is wrong".to_string()
        },
        JsonRejection::JsonSyntaxError(_) => HandlerError {
            status: StatusCode::BAD_REQUEST,
            code: "WRONG_PAYLOAD".to_string(),
            message: "The payload is wrong".to_string()
        },
        JsonRejection::MissingJsonContentType(_) => HandlerError {
            status: StatusCode::BAD_REQUEST,
            code: "WRONG_PAYLOAD".to_string(),
            message: "The payload is wrong".to_string()
        },
        JsonRejection::BytesRejection(_) => HandlerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "UNKNOWN".to_string(),
            message: "An unknown error has occured.".to_string()
        },
        _ => HandlerError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "UNKNOWN".to_string(),
            message: "An unknown error has occured.".to_string()
        }
    }
}
