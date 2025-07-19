use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, errors::Error};
use serde::{Serialize, de::DeserializeOwned};

pub async fn decode_jwt<T>(token: &str, secret: &str) -> Result<TokenData<T>, Error>
where
    T: DeserializeOwned,
{
    jsonwebtoken::decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

pub async fn encode_jwt<T>(claims: &T, secret: &str) -> Result<String, Error>
where
    T: Serialize,
{
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
