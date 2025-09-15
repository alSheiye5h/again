use crate::models::Jwt_struct::{Claims, Keys};
use jsonwebtoken::{decode, TokenData, Validation};

pub fn validate_jwt(
    token: &str,
    keys: &Keys,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &keys.decoding, &Validation::default())
}