use rocket::{ catch, catchers };

use diesel_demo::{
    api::user_api::*,
    ApiError, ApiErrorResponse, PgConnection
};

#[catch(422)]
fn unprocessable_entity() -> ApiError {
    ApiErrorResponse::validation_error("Unprocessable Entity")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
        .mount("/users", rocket::routes![list_all, list_by_id, create, update, delete])
        .register("/users", catchers![unprocessable_entity])
}