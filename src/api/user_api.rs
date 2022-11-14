use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use diesel::prelude::*;

use crate::{
    schema::users,
    models::user_model::{User, NewUser, UpdateUser},
    ApiError, ApiErrorResponse, PgConnection
};


#[rocket::get("/")]
pub async fn list_all(conn: PgConnection) -> Result<Json<Vec<User>>, ApiError> {    
    let result = conn.run(|c| {
        users::table.load(c)
    }).await;

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

#[rocket::get("/<id>")]
pub async fn list_by_id(conn: PgConnection, id: i32) -> Result<Json<User>, ApiError> {
    let result = conn.run(move |c| 
        users::table
            .find(id)
            .get_result::<User>(c)
    ).await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}

#[rocket::post("/<id>", data = "<users>", format = "json")]
pub async fn update(conn: PgConnection, id: i32, users: Json<UpdateUser>) -> Result<Json<User>, ApiError> {
    let result = conn.run(move |c| {
        diesel::update(users::table.find(id))
            .set(&users.0)
            .get_result::<User>(c)
    }).await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}

#[rocket::delete("/<id>")]
pub async fn delete(connection: PgConnection, id: i32) -> Result<NoContent, ApiError> {
    let result = connection.run(move |c| {
        diesel::delete(users::table.find(id))
            .execute(c)
    }).await;

    match result {
        Ok(_) => Ok(NoContent),
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}