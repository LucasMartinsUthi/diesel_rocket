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
    }, http::Status
};
use rocket::request::{FromRequest, Request, Outcome};
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
    InvalidToken(Json<String>),
}

#[derive(Debug)]
pub struct MonstaToken(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MonstaToken {
    type Error = ApiError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == "087FDE79-7B55-4C8C-A5D2-00768CFEB1B7"
        }

        match request.headers().get_one("X-Monsta-Token") {
            Some(key) if is_valid(key) => Outcome::Success(MonstaToken(key.to_string())),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiError::InvalidToken(Json("Invalid Token".to_string())))),
            None => Outcome::Failure((Status::BadRequest, ApiError::InvalidToken(Json("Missing 2 Token".to_string())))),
        }
    }
            
}

