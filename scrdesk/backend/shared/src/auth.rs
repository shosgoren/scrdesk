use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{TenantId, UserId, UserRole};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub tenant_id: TenantId,
    pub role: UserRole,
    pub exp: i64,
    pub iat: i64,
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiry: i64,
    refresh_token_expiry: i64,
}

impl JwtManager {
    pub fn new(secret: &str, access_expiry: i64, refresh_expiry: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_expiry: access_expiry,
            refresh_token_expiry: refresh_expiry,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        role: UserRole,
    ) -> Result<String> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(self.access_token_expiry)).timestamp();

        let claims = Claims {
            sub: user_id,
            tenant_id,
            role,
            exp,
            iat: now.timestamp(),
            token_type: TokenType::Access,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))
    }

    pub fn generate_refresh_token(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        role: UserRole,
    ) -> Result<String> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(self.refresh_token_expiry)).timestamp();

        let claims = Claims {
            sub: user_id,
            tenant_id,
            role,
            exp,
            iat: now.timestamp(),
            token_type: TokenType::Refresh,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| Error::Jwt(e))
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let claims = self.verify_token(token)?;
        if claims.token_type != TokenType::Access {
            return Err(Error::Authentication("Invalid token type".to_string()));
        }
        Ok(claims)
    }

    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims> {
        let claims = self.verify_token(token)?;
        if claims.token_type != TokenType::Refresh {
            return Err(Error::Authentication("Invalid token type".to_string()));
        }
        Ok(claims)
    }
}
