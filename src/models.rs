use crate::token::{create_claims, create_jwt_secret, encode_token, Claims, TokenError};
use uuid::Uuid;

struct App {
    id: Uuid,
    jwt_secret: String,
}

#[derive(Debug)]
enum AppError {
    TokenError(TokenError),
}

impl App {
    fn encode_jwt(&self) -> Result<String, AppError> {
        let claims = create_claims(&self.id.to_simple().to_string());
        encode_token(&claims, &self.jwt_secret).map_err(|e| AppError::TokenError(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_app_encode_jwt() {
        let app = App {
            id: Uuid::new_v4(),
            jwt_secret: create_jwt_secret(),
        };
        let token = app.encode_jwt().unwrap();
        assert_eq!(token.split(".").collect::<Vec<&str>>().len(), 3);
    }
}
