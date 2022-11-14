use rocket::{
    serde::json::Json,
    catch, catchers
};

use diesel_demo::{
    api::user_api::*,
    ApiError, ApiErrorResponse, PgConnection
};

#[catch(422)]
fn unprocessable_entity() -> ApiError {
    ApiError::ValidationError(Json(ApiErrorResponse {
        message: "Unprocessable Entity".to_string(),
    }))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
        .mount("/users", rocket::routes![list, create])
        .register("/users", catchers![unprocessable_entity])
}