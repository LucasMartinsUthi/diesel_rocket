use rocket::serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable, AsChangeset};

use crate::schema::users;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: String,
}