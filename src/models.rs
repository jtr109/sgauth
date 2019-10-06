use crate::token::AppError as TokenError;
use crate::token::{create_claims, encode_token, Claims};
use uuid::Uuid;

struct App {
    id: Uuid,
    jwt_secret: String,
}

enum AppError {
    TokenError(TokenError),
}

impl App {
    fn encode_jwt(&self) -> Result<String, AppError> {
        let claims = create_claims(&self.id.to_simple().to_string());
        encode_token(&claims, &self.jwt_secret).map_err(|e| AppError::TokenError(e))
    }
}
