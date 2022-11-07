#[macro_use] extern crate rocket;

use diesel_rocket::{PgConnection};

//TODO
// mount user once

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
        // .mount("/", routes![create_user])
        // .mount("/", routes![get_user])
}
