extern crate sgauth;

use actix_web::{web, App, HttpResponse, HttpServer};
use sgauth::handlers;

fn main() {
    HttpServer::new(|| {
        App::new().service(web::scope("/app").route("/auth", web::get().to(|| HttpResponse::Ok())))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}