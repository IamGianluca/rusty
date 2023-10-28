use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::adapters::user_repository::UserRepository;

pub fn get_secret_key() -> String {
    use dotenvy::dotenv;
    dotenv().ok();
    env::var("JWT_SECRET_KEY").expect("SECRET_KEY environment is not set.")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // subject
    pub iat: usize,  // issued at
    pub exp: usize,  // expiration time
}

pub fn generate_jwt_token(user_id: &i32) -> String {
    let secret = get_secret_key();
    let res = create_token(user_id.to_string().as_str(), &secret.as_bytes(), 60).unwrap();
    res
}

pub fn create_token(
    user_id: &str,
    secret: &[u8],
    expires_in_seconds: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidSubject.into());
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expires_in_seconds)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: Into<String>>(token: T, secret: &[u8]) -> Result<String, &'static str> {
    let decoded = decode::<TokenClaims>(
        &token.into(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );
    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err("Invalid token"),
    }
}

pub fn validate_token(token: &str) -> Result<(), String> {
    if token == get_secret_key() {
        Ok(())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn authenticate_user(
    username: &str,
    password: &str,
    repo: &mut dyn UserRepository,
) -> Option<i32> {
    match repo.get_user_by_name(username) {
        Some(user) => {
            let credentials = repo.get_credential_by_user_id(user.id).unwrap();
            if credentials.password == password {
                return Some(user.id);
            } else {
                return None;
            }
        }
        None => return None,
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::user_repository::DbUserRepository;
    use crate::adapters::utils::init_db;
    use crate::service_layer::authenticate::authenticate_user;
    use crate::service_layer::authenticate::{create_token, decode_token};
    use crate::service_layer::service::create_user;

    #[test]
    fn test_service_authenticate_user() {
        // given
        let conn = &mut init_db();
        let mut repo = DbUserRepository { conn };

        // when
        let response = authenticate_user("John Doe", "password", &mut repo);

        // then
        assert!(response.is_none());

        // given
        let user_id = create_user("John Doe", "johndoe@example.com", "password", &mut repo);

        // when
        let response = authenticate_user("John Doe", "password", &mut repo);

        // then
        assert_eq!(response.unwrap(), user_id);
    }

    #[test]
    fn test_create_and_decoded_valid_token() {
        // given
        let user_id = "user123";
        let secret = b"my-secret-key";

        // when
        let token = create_token(user_id, secret, 60).unwrap();
        let decoded_user_id = decode_token(&token, secret).unwrap();

        // then
        assert_eq!(decoded_user_id, user_id);
    }
}
