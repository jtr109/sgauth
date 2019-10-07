extern crate sgauth;

use sgauth::models::{establish_connection, App};
use uuid::Uuid;

// create jwt from app
fn main() {
    // let id = Uuid::new_v4();
    let id = Uuid::parse_str("a3e1d04b629d4faaac5ab7a77b41be39").unwrap();
    let conn = establish_connection();
    let app = App::get_by_id(&conn, id).unwrap().unwrap();
    println!("app token is {:?}", app.encode_jwt().unwrap());
}
