use bcrypt::{hash, verify, BcryptError};

pub fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password, 10)
}

pub fn verify_password(password: String, hash: String) -> Result<bool, BcryptError> {
    verify(password, hash.as_str())
}
