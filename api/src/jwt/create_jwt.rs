use crate::models::jwtStruct::{Claims, Keys};
use jsonwebtoken::{encode, Header};

pub fn create_jwt(user_id: &str, keys: &Keys) -> Result<String, jsonwebtoken::errors::Error> {
    // Token expires in 24 hours
    let claims = Claims::new(user_id, 24);
    encode(&Header::default(), &claims, &keys.encoding)
}
