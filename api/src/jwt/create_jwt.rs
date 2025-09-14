use api::models::jwtStruct::{Claims, Keys};
use jsonwebtoken::{encode, decode, Header, Validation, TokenData, errors::Result};

pub fn create_jwt(user_id: &str, keys: &Keys) -> Result<String> {
    let claims = Claims::new(user_id, 24); // 24h token
    encode(&Header::default(), &claims, &keys.encoding)
}
