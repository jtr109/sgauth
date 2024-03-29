use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use crate::schema::app;
use crate::token::{
    create_claims, create_jwt_secret, decode_token, encode_token, get_sub_without_verification,
    TokenError,
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Queryable, Debug)]
pub struct App {
    pub id: Uuid,
    pub jwt_secret: String,
}

#[derive(Insertable)]
#[table_name = "app"]
struct NewApp {
    jwt_secret: String,
}

#[derive(Debug)]
pub enum AppError {
    TokenError(TokenError),
    DieselError(DieselError),
    UuidError(uuid::parser::ParseError),
}

impl App {
    pub fn encode_jwt(&self) -> Result<String, AppError> {
        let claims = create_claims(&self.id.to_simple().to_string());
        encode_token(&claims, &self.jwt_secret).map_err(|e| AppError::TokenError(e))
    }

    pub fn get_by_id(conn: &PgConnection, id: &Uuid) -> Result<Option<App>, AppError> {
        app::table
            .filter(app::dsl::id.eq(id))
            .limit(1)
            .first::<App>(conn)
            .optional()
            .map_err(|e| AppError::DieselError(e))
    }

    pub fn create(conn: &PgConnection) -> Result<Self, AppError> {
        let new_app = NewApp {
            jwt_secret: create_jwt_secret(),
        };
        diesel::insert_into(app::table)
            .values(&new_app)
            .get_result(conn)
            .map_err(|e| AppError::DieselError(e))
    }

    pub fn get_from_jwt(conn: &PgConnection, encoded: &str) -> Result<Option<App>, AppError> {
        let sub = get_sub_without_verification(encoded).map_err(|e| AppError::TokenError(e))?;
        let id = Uuid::parse_str(&sub).map_err(|e| AppError::UuidError(e))?;
        match Self::get_by_id(conn, &id) {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(app)) => {
                decode_token(encoded, &app.jwt_secret).map_err(|e| AppError::TokenError(e))?; // validate jwt
                Ok(Some(app))
            }
        }
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
