use jsonwebtoken as jwt;
use jsonwebtoken::{dangerous_unsafe_decode, decode, encode, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

fn create_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

pub fn create_jwt_secret() -> String {
    create_random_string(32)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
}

pub fn create_claims(sub: &str) -> Claims {
    Claims {
        sub: sub.to_string(),
    }
}

#[derive(Debug)]
pub enum TokenError {
    JwtError(jwt::errors::Error),
}

impl From<jwt::errors::Error> for TokenError {
    fn from(error: jwt::errors::Error) -> Self {
        TokenError::JwtError(error)
    }
}

pub fn encode_token(claims: &Claims, secret: &str) -> Result<String, TokenError> {
    encode(&Header::default(), claims, secret.as_bytes()).map_err(|e| TokenError::JwtError(e))
}

// decode json web token
// 由于目前 token 中不需要 exp, 忽略 exp validation
// 方法见: https://github.com/Keats/jsonwebtoken/issues/65
pub fn decode_token(encoded: &str, secret: &str) -> Result<Claims, TokenError> {
    let valication = Validation {
        validate_exp: false,
        ..Validation::default()
    };
    decode::<Claims>(encoded, secret.as_bytes(), &valication)
        .map(|data| data.claims.into())
        .map_err(|e| TokenError::JwtError(e))
}

// *危险*
// 未验证解析 payload 信息, 没有安全措施
// https://docs.rs/jsonwebtoken/6.0.1/jsonwebtoken/fn.dangerous_unsafe_decode.html
pub fn get_sub_without_verification(encoded: &str) -> Result<String, TokenError> {
    dangerous_unsafe_decode::<Claims>(encoded)
        .map(|data| data.claims.sub)
        .map_err(|e| TokenError::JwtError(e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_random_string() {
        let length = rand::thread_rng().gen_range(8, 32);
        let rand_string = create_random_string(length);
        assert_eq!(rand_string.len(), length);
    }

    #[test]
    fn test_encode_token() {
        let secret = "your-256-bit-secret";
        let expected = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.9nQevcPZ--Asqxinx5l1pgqRf0tCGR6Wvw_AvVN5CDA";
        let claims = create_claims("1234567890");
        assert_eq!(encode_token(&claims, &secret).unwrap(), expected);
    }

    #[test]
    fn test_decode_token() {
        let encoded = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.9nQevcPZ--Asqxinx5l1pgqRf0tCGR6Wvw_AvVN5CDA";
        let secret = "your-256-bit-secret";
        // let claims = decode_token(&encoded, &secret).unwrap();
        let claims = decode_token(&encoded, &secret).unwrap();
        assert_eq!(claims.sub, "1234567890");
    }
}
