use rocket::{ catch, catchers };

use diesel_demo::{
    api::*,
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
        .mount("/", user_api::routes())
        .mount("/", token_api::routes())
        .register("/", catchers![unprocessable_entity])
        .register("/", token_api::catchers())
}