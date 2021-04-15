use argonautica::{Hasher, Verifier};
use super::env::{ PASSWORD_SECRET_KEY, JWT_SECRET_KEY };

use actix_web::HttpRequest;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct ContextToken {
    pub user_id: Uuid,
    pub token: String,
    // pub user: User
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub id: String,
    pub exp: i64,
	pub sub: String
}

pub fn hash_password(password: &str) -> String {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
		.unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
		.unwrap()
}


pub fn create_token(user_id: uuid::Uuid, username: String) -> String {
    let exp_time = Local::now() + Duration::days(7);

    let claims = Claims {
        id: user_id.to_string(),
        exp: exp_time.timestamp(),
		sub: username
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()))
        .expect("Can't create token")
}

pub fn get_jwt_payload(http_request: HttpRequest) -> Option<ContextToken> {
	http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| header_value.to_str().ok().map(|s| {
            let jwt_start_index = "Bearer ".len();
            let jwt = s[jwt_start_index..s.len()].to_string();
            let token_data = decode_token(&jwt);
			return ContextToken {
                token: jwt,
                user_id: Uuid::parse_str(&token_data.claims.id).unwrap()
            }
        }))
}

fn decode_token(token: &str) -> TokenData<Claims> {
    decode::<Claims>(&token, &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()), &Validation::default())
        .expect("Can't decode token")
}