use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error};
use serde::{Deserialize, Serialize};

use crate::models::user::UserDTO;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role_name: String,
    pub exp: usize,
}

pub fn encode(user: &UserDTO, key: &str) -> Result<String, Error> {
    let token = jsonwebtoken::encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret(key.as_ref()),
    );

    token
}

pub fn decode(token: &str, key: &str) -> Result<TokenData<UserDTO>, Error> {
    let token = jsonwebtoken::decode::<UserDTO>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    );

    token
}
