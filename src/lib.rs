#[macro_use] extern crate rocket;

use rocket_sync_db_pools::database;

pub mod schema;
pub mod api;
pub mod models;

#[database("db")]
pub struct PgConnection(rocket_sync_db_pools::diesel::PgConnection);