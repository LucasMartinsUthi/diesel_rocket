use serde::{Serialize, Deserialize};

use diesel::prelude::*;
use crate::schema::users;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub location: String,
    pub title: String,
}