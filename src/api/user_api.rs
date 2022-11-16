use rocket::{
    response::{status::{Created, NoContent}},
    serde::json::Json,
};

use rocket::response::{Redirect};
use rocket::http::{Cookie, CookieJar};
use rocket::uri;
use rocket::routes;

use diesel::prelude::*;


use crate::{
    schema::users,
    models::user_model::{User, UserLogin, UserCookie},
    ApiError, ApiErrorResponse, PgConnection
};

#[rocket::get("/")]
pub async fn list_all(user_cookie: UserCookie, conn: PgConnection)  -> Result<Json<Vec<User>>, ApiError> { 
    
    println!("user_cookie: {:?}", user_cookie);
    
    let result = conn.run(move |c| {
        users::table
            .find(user_cookie.0)
            .get_result::<User>(c)
    }).await;

    match result {
        Ok(users) => {
            // if users.len() == 0 {
            //     return Err(ApiErrorResponse::new("No users found"));
            // }
            Ok(Json(vec![users]))
        }
        Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
    }
}

#[rocket::get("/", rank = 2)]
pub async fn no_auth() -> NoContent {    
    NoContent
}

#[rocket::get("/login")]
pub fn login(_user: UserCookie) -> Redirect {
    Redirect::to(uri!(list_all))
}

#[rocket::post("/login", data = "<login>")]
pub async fn post_login(conn: PgConnection, jar: &CookieJar<'_>, login: Json<UserLogin>) -> Result<Redirect, ApiError> {
    let login = login.into_inner();

    let result = conn.run(move |c| {
        users::table.filter(users::login.eq(&login.login)).first::<User>(c)
    }).await;

    if let Ok(user) = result {
        if user.password == login.password {
            jar.add_private(Cookie::new("user_id", user.id.to_string()));

            return Ok(Redirect::to(uri!(list_all)))
        }

        return Err(ApiErrorResponse::new("Invalid password"))
    }
     
    return Err(ApiErrorResponse::new("Invalid login"));
}
#[rocket::post("/register", data = "<users>", format = "json")]
pub async fn register(conn: PgConnection, users: Json<UserLogin>) -> Result<Created<Json<User>>, ApiError> {
    let result = conn.run(move |c| {
        diesel::insert_into(users::table)
            .values(&users.0)
            .get_result(c)
    }).await;

    match result {
        Ok(user) => Ok(Created::new("/").body(Json(user))),
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

// #[rocket::post("/<id>", data = "<users>", format = "json")]
// pub async fn update(conn: PgConnection, id: i32, users: Json<UpdateUser>) -> Result<Json<User>, ApiError> {
//     let result = conn.run(move |c| {
//         diesel::update(users::table.find(id))
//             .set(&users.0)
//             .get_result::<User>(c)
//     }).await;

//     match result {
//         Ok(user) => Ok(Json(user)),
//         Err(e) => Err(ApiErrorResponse::new(&e.to_string())),
//     }
// }

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

#[rocket::post("/logout")]
fn logout(jar: &CookieJar<'_>) -> NoContent {
    jar.remove_private(Cookie::named("user_id"));
    NoContent
}


pub fn routes() -> Vec<rocket::Route> {
    routes![list_all, no_auth, login, post_login, logout, register]
}