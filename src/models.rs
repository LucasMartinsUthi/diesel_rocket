use diesel::prelude::*;

use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub title: String,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub title: &'a str,
}