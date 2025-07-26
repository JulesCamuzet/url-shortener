use sqlx::{PgPool, Error};

use crate::models::user::UserDb;

pub async fn get_one_user_by_email(email: &str, pool: &PgPool) -> Result<Option<UserDb>, Error> {
    let query = sqlx::query_as::<_, UserDb>(
        "SELECT
            id,
            email,
            password,
            reset_password_token,
            verification_token,
            is_verified,
            created_at
        FROM users WHERE email = $1",
    ).bind(email);

    let result = query.fetch_optional(pool).await;

    match result {
        Err(e) => {
            eprintln!("Error while get_one_user_by_email. Email : {email}. Error {e}");
            Err(e)
        },
        Ok(user) => Ok(user)
    }
}
