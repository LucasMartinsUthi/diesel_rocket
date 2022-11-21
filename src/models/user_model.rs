use rocket::serde::{Deserialize, Serialize};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::FromForm;

use diesel::{Insertable, Queryable, AsChangeset};

use crate::schema::users;

#[derive(Debug)]
pub struct UserCookie(pub i32);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserCookie {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserCookie, Self::Error> {
        
        request.cookies().get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(UserCookie)
            .or_forward(())
    }
}

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Insertable, Debug, Queryable, FromForm)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UserLogin {
    pub login: String,
    pub password: String,
}

// #[derive(Deserialize, AsChangeset, Debug)]
// #[serde(crate = "rocket::serde")]
// #[table_name = "users"]
// pub struct UpdateUser {
//     pub login: String,
// }