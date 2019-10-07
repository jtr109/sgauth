extern crate sgauth;

use sgauth::models::{establish_connection, App};
use std::env;
use uuid::Uuid;

// create jwt from app
fn main() {
    let args: Vec<String> = env::args().collect();
    let id = Uuid::parse_str(&args[1]).unwrap();
    let conn = establish_connection();
    let app = App::get_by_id(&conn, &id).unwrap().unwrap();
    println!("app token is\n{}", app.encode_jwt().unwrap());
}
