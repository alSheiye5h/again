use crate::models::jwt_struct::{Claims, Keys};
use jsonwebtoken::{encode, Header};

/// Creates a JSON Web Token (JWT).
pub fn create_jwt(user_id: &str, keys: &Keys) -> Result<String, jsonwebtoken::errors::Error> {
    // The token will be valid for 24 hours.
    let claims = Claims::new(user_id, 24);
    encode(&Header::default(), &claims, &keys.encoding)
}
