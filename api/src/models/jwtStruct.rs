use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, DecodingKey};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user id or email)
    pub exp: usize,  // expiration time
}

impl Claims {
    pub fn new(user_id: &str, expiry_hours: i64) -> Self {
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(expiry_hours))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            sub: user_id.to_owned(),
            exp,
        }
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &str) -> Self {
        Keys {
            encoding: EncodingKey::from_secret(secret.as_ref()),
            decoding: DecodingKey::from_secret(secret.as_ref()),
        }
    }
}