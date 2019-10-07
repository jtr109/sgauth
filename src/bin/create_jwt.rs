extern crate sgauth;

use sgauth::models::{establish_connection, App};

// create jwt from app
fn main() {
    let conn = establish_connection();
    let app = App::create(&conn);
    println!("app created: {:?}", app);
}
