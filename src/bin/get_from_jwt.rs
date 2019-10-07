extern crate sgauth;

use sgauth::models::{establish_connection, App};
use std::env;

// get app from encode jwt
fn main() {
    let args: Vec<String> = env::args().collect();
    let encoded = &args[1]; // encoded jwt
    let conn = establish_connection();
    let app = App::get_from_jwt(&conn, &encoded).unwrap().unwrap();
    println!("app is {:?}", app);
}
