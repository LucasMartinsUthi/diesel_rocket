use crate::PgConnection;
use crate::schema::users;
use crate::models::{NewUser, User};

use rocket::{http::Status, serde::json::Json};

#[post("/user", data = "<new_user>")]
pub fn create_user(conn: &mut PgConnection, new_user: Json<NewUser>) -> Result<Json<String>, Status> {
    // use crate::schema::users::dsl::*;

    let data = NewUser {
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };



    let result = diesel::insert_into(users::table)
        .values(&data)
        .get_result::<User>(conn);

    Ok(Json(result))
}

// #[get("/users")]
// pub fn get_user(path: String) -> Result<Json<User>, Status> {
    
// }
    