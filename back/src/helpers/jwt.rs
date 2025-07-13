use serde::{Deserialize, Serialize};
use jsonwebtoken::{
    decode,
    encode,
    errors::Error,
    DecodingKey,
    EncodingKey,
    Header,
    TokenData,
    Validation
};

#[derive(Serialize, Deserialize)]
pub struct Claim {
    pub email: String
}

pub fn get_jwt(claim: Claim, private_key: String) -> Result<String, Error> {
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(private_key.as_bytes());
    let token = encode(&header, &claim, &encoding_key);
    return token;
}

pub fn verify_jwt(token: String, private_key: String) -> Result<TokenData<Claim>, Error> {
    let decoding_key = DecodingKey::from_secret(private_key.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let result = decode::<Claim>(&token, &decoding_key, &validation);
    return result;
}
