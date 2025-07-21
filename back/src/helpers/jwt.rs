use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{
    decode,
    encode,
    errors::Error,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};

// TODO should maybe find a better place to declare the Claim struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Claim {
    pub email: String,
    pub exp: i64
}

pub fn get_jwt(claim: Claim, private_key: String) -> Result<String, Error> {
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(private_key.as_bytes());
    let token = encode(&header, &claim, &encoding_key);
    return token;
}

pub fn verify_jwt(token: &str, private_key: String) -> Result<Claim, Error> {
    let decoding_key = DecodingKey::from_secret(private_key.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let result = decode::<Claim>(&token, &decoding_key, &validation)?;
    Ok(result.claims)
}
