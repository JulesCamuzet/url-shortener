use sqlx::PgPool;

use crate::db::helpers::fetch_one::fetch_one_from_db;

pub async fn get_one_user_by_email(email: &str, pool: &PgPool) {
    let result = fetch_one_from_db(
        "SELECT id, email, password, reset_password_token, verification_token, is_verified, created_at FROM users WHERE email = $1",
        [email].to_vec(),
        pool
    ).await;
}
