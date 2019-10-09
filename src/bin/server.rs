use actix_web::{web, App, HttpResponse, HttpServer};

fn main() {
    HttpServer::new(|| {
        App::new().service(web::scope("/app").route("/auth", web::get().to(|| HttpResponse::Ok())))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
