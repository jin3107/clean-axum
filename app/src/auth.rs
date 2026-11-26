use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use models::domains::role::Role;

use crate::error::AuthError;

pub const TOKEN_TTL_SECS: u64 = 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub tenant_id: Uuid,
    pub role: Role,
    pub exp: u64,
}

#[derive(Clone)]
pub struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtKeys {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn issue(&self, user_id: Uuid, tenant_id: Uuid, role: Role) -> Result<String, AuthError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AuthError::Internal)?
            .as_secs();
        let claims = Claims {
            sub: user_id,
            tenant_id,
            role,
            exp: now + TOKEN_TTL_SECS,
        };
        encode(&Header::default(), &claims, &self.encoding).map_err(|_| AuthError::Internal)
    }

    /// Accepts HS256 only and rejects expired tokens.
    pub fn verify(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(token, &self.decoding, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AuthError::InvalidToken)
    }
}

// Argon2 is deliberately slow, so it runs off the async worker threads.
pub async fn hash_password(password: String) -> Result<String, AuthError> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| AuthError::Internal)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}

pub async fn verify_password(password: String, hash: String) -> Result<(), AuthError> {
    tokio::task::spawn_blocking(move || {
        let parsed = PasswordHash::new(&hash).map_err(|_| AuthError::Internal)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| AuthError::InvalidCredentials)
    })
    .await
    .map_err(|_| AuthError::Internal)?
}
