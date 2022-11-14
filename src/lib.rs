pub mod api;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
use rocket::{
    Responder,
    serde::{
        Deserialize, Serialize, 
        json::Json
    }
};
use rocket_sync_db_pools::database;

#[database("db")]
pub struct PgConnection(diesel::PgConnection);


#[derive(Responder, Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiErrorResponse {
    pub message: String,
}

impl ApiErrorResponse {
    pub fn new(message: &str) -> ApiError {
        ApiError::StandardError(Json(ApiErrorResponse {
            message: message.to_string(),
        }))
    }

    pub fn validation_error(message: &str) -> ApiError {
        ApiError::ValidationError(Json(ApiErrorResponse {
            message: message.to_string(),
        }))
    }
}

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    StandardError(Json<ApiErrorResponse>),
    #[response(status = 422, content_type = "json")]
    ValidationError(Json<ApiErrorResponse>),
}

