extern crate sgauth;

use sgauth::models::{establish_connection, App};

fn main() {
    let conn = establish_connection();
    let app = App::create(&conn).unwrap();
    println!("app created: {}", app.id.to_simple().to_string());
}
