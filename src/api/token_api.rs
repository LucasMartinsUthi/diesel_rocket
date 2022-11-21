use rocket::{
    response::{status::{Created, NoContent}},
    serde::json::Json,
};
use rocket::{ catch, catchers};
use rocket::Request;
use rocket::http::Status;

use crate::{ApiError, ApiErrorResponse};

use rocket::routes;

use crate::MonstaToken;

// #[catch(default)]
// fn default_catcher_token(status: Status, req: &Request) -> ApiError {
//     println!("status: {:?}", status);
//     println!("req: {:?}", req);
    
//     ApiError::InvalidToken(Json("a".to_string()))
// }

//catcher for 400
#[catch(400)]
pub fn bad_request(status: Status, req: &Request) -> ApiError {
    println!("status: {:?}", status);
    println!("req: {:?}", req);

    ApiError::InvalidToken(Json("a".to_string()))
}

#[rocket::get("/token")]
pub async fn token(token: Result<MonstaToken, ApiError>) -> Result<Json<String>, ApiError> {    
    match token {
        Ok(token) => {
            println!("token: {:?}", token);
            Ok(Json("Valid Token".to_string()))
        }
        Err(e) => Err(e)
    }
}

// #[rocket::get("/token", rank = 2)]
// pub async fn no_token() -> Json<String> {    
//     Json("Missing Token".to_string())
// }


pub fn routes() -> Vec<rocket::Route> {
    routes![token]
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![bad_request]
}