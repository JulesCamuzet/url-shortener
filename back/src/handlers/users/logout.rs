use axum::{
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse, Response},
};

pub async fn handle_logout() -> Response {
    let past_date = chrono::offset::Local::now() - chrono::Duration::minutes(69);
    let expired_cookie = past_date.to_rfc2822();

    let headers = AppendHeaders([(
        SET_COOKIE,
        format!(
            "auth=expired; Expires={}",
            expired_cookie
        ),
    )]);

    (StatusCode::NO_CONTENT, headers).into_response()
}
