use crate::common::errors::ApiError;
use crate::config::CONFIG;
use argon2rs::argon2i_simple;
use chrono::{Duration as ChronDur, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub sub: String,
    pub email: Option<String>,
    pub username: Option<String>,
    pub mobile: Option<String>,
    exp: i64,
    aud: String,
    iat: i64,
    iss: String,
    jti: uuid::Uuid,
}

impl PrivateClaim {
    pub fn new(
        user_id: String,
        email: Option<String>,
        username: Option<String>,
        mobile: Option<String>,
    ) -> Self {
        Self {
            sub: user_id,
            email,
            username,
            mobile,
            exp: (Utc::now() + ChronDur::hours(CONFIG.security.jwt_expiration)).timestamp(),
            aud: "".to_string(),
            iat: Utc::now().timestamp(),
            iss: CONFIG.security.jwt_issuer.to_string(),
            jti: uuid::Uuid::new_v4(),
        }
    }
}

/// Create a json appstate token (JWT)
pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ApiError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.security.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ApiError::CannotEncodeJwtToken(e.to_string()))
}

/// Decode a json appstate token (JWT)
pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ApiError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.security.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ApiError::CannotDecodeJwtToken(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// auth_salt is environment-configured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.security.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed.as_str());
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let email = Option::from("test@test.com".to_string());
        let private_claim = PrivateClaim::new("1234".parse().unwrap(), email, None, None);
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let email = Option::from("test@test.com".to_string());
        let private_claim = PrivateClaim::new("3467".parse().unwrap(), email, None, None);
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
