use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use jwt::VerifyWithKey;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use actix_web::HttpRequest;

use crate::internal::user::entity::token;

#[derive(Debug, PartialEq)]
pub enum AccessTokenError {
    TokenInvalid,
    ExpiredToken,
}

pub struct AccessTokenResult {
    pub username: String,
    pub remaining_time: u64,
}

pub fn get_time_sec() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

async fn verify_access_token(access_token: &str) -> Result<AccessTokenResult, AccessTokenError> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(token::TOKEN_SECRET_KEY.as_ref()).unwrap();
    let claims: BTreeMap<String, String> = match access_token.verify_with_key(&key) {
        Ok(res) => {
            res
        },
        Err(_err) => {
            return Err(AccessTokenError::TokenInvalid);
        }
    };

    let token_expire: u64 = claims["created_time"].parse().unwrap();
    if get_time_sec() > token_expire {
        return Err(AccessTokenError::ExpiredToken);
    }

    let remaining_time: u64 = token_expire - get_time_sec();
    let response = AccessTokenResult {
        username: claims["username"].to_string(),
        remaining_time,
    };

    Ok(response)
}

pub async fn is_unauthorized(req: &HttpRequest) -> Result<AccessTokenResult, AccessTokenError> {
    let token = match req.headers().get("authorization") {
        Some(res) => {
            res.to_str().unwrap()
        },

        None => {
            return Err(AccessTokenError::TokenInvalid);
        }
    };
    let token = &token[7..];

    return match verify_access_token(token).await {
        Ok(res) => {
            Ok(res)
        },
        Err(err) => {
            Err(err)
        }
    };
}
