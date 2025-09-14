use api::models::jwtStruct::{Claims, Keys};
use jsonwebtoken::{encode, decode, Header, Validation, TokenData, errors::Result};


pub fn validate_jwt(token: &str, keys: &Keys) -> Result<TokenData<Claims>> {
    decode::<Claims>(token, &keys.decoding, &Validation::default())
}