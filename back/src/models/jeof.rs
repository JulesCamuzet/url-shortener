pub struct User {
    pub id: i32,
    pub email: String,
    pub verification_token: String,
    pub is_verified: bool,
    pub password: String,
    pub reset_password_token: Option<String>,
    pub created_at: String
}

pub fn test() {1}
