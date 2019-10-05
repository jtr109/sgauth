use jsonwebtoken as jwt;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

fn create_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

fn create_jwt_secret() -> String {
    create_random_string(32)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

fn create_claims(sub: &str) -> Claims {
    Claims {
        sub: sub.to_string(),
    }
}

fn encode_token(claims: &Claims, secret: &str) -> jwt::errors::Result<String> {
    encode(&Header::default(), claims, secret.as_bytes())
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
}
