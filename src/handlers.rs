use crate::models::{establish_connection, App, AppError};
use actix_web::{HttpRequest, HttpResponse};
use regex::Regex;

const AUTHORIZATION: &str = "Authorization";

fn get_jwt_from_request(req: &HttpRequest) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Bearer (?P<token>\w+\.\w+\.\w+)$").unwrap();
    }
    let auth = req.headers().get(AUTHORIZATION)?;
    match auth.to_str() {
        Err(_) => None,
        Ok(s) => RE
            .captures(s)
            .and_then(|cap| cap.name("token").map(|t| t.as_str().to_string())),
    }
}

pub fn authentication(req: HttpRequest) -> HttpResponse {
    let unauthorized = HttpResponse::new(http::StatusCode::UNAUTHORIZED);
    let token = match get_jwt_from_request(&req) {
        None => return unauthorized,
        Some(t) => t,
    };
    let conn = establish_connection();
    let app = match App::get_from_jwt(&conn, &token) {
        Err(AppError::DieselError(_)) => {
            return HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) => return unauthorized,
        Ok(a) => a,
    };
    match app {
        Some(a) => HttpResponse::Ok()
            .header("X-UID", a.id.to_simple().to_string())
            .body(""),
        None => unauthorized,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;

    #[test]
    fn test_get_jwt_from_request() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let auth = format!("Bearer {}", token);
        let req = test::TestRequest::with_header(AUTHORIZATION, auth).to_http_request();

        assert_eq!(get_jwt_from_request(&req), Some(token.to_string()));
    }
}
