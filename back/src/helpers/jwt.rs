use jsonwebtoken::{encode, EncodingKey, Header, errors::Error};
use serde::Serialize;

#[derive(Serialize)]
pub struct Claim {
    email: String
}

pub fn get_jwt(claim: Claim, private_key: String) -> Result<String, Error> {
    let header = Header::new(jsonwebtoken::Algorithm::ES256);
    let encoding_key = EncodingKey::from_secret(private_key.as_bytes());
    let token = encode(&header, &claim, &encoding_key);
    return token;
}

pub fn verify_jwt() {
    
}
