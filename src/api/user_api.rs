use rocket::{
    response::status::{Created},
    serde::json::Json,
};

use diesel::prelude::*;

use crate::{
    schema::users,
    models::user_model::{User, NewUser},
    ApiError, ApiErrorResponse, PgConnection
};


#[rocket::get("/")]
pub async fn list(conn: PgConnection) -> Result<Json<Vec<User>>, ApiError> {
    let result = conn.run(|c| users::table.load(c)).await;

    match result {
        Ok(users) => {
            if users.len() == 0 {
                return Err(ApiErrorResponse::new("No users found"));
            }
            Ok(Json(users))
        }
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}

#[rocket::post("/", data = "<users>", format = "json")]
pub async fn create(conn: PgConnection, users: Json<NewUser>) -> Result<Created<Json<User>>, ApiError> {
    let result = conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&users.0)
            .get_result(c)
    }).await;

    match result {
        Ok(user) => Ok(Created::new("/users").body(Json(user))),
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}