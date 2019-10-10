extern crate sgauth;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger;
use sgauth::handlers::authentication;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/app").route("/auth", web::get().to(authentication)))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
