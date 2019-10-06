use app::{create_claims, encode_token, AppError, Claims};

struct App {
    id: String,
    jwt_secret: String,
}

impl App {
    fn encode_jwt(&self) -> String {
        let claims = create_claims(self.id);
        encode_token(&claims, self.jwt_secret)
    }
}
