use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::{Request};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::api::config::Config; // your application's configuration module

#[derive(Debug, Deserialize)]
pub struct JwtGuard {
    pub user_id: String,
}

#[derive(Debug, PartialEq)]
pub enum JwtGuardError {
    MissingToken,
    InvalidToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtGuard {
    type Error = JwtGuardError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {

        let config = Config::new();

        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = auth_header.trim_start_matches("Bearer ");
                let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

                // Try to decode the JWT payload into a HashMap
                let decoded_token: Result<HashMap<String, Value>, _> = decode::<HashMap<String, Value>>(
                    token,
                    &DecodingKey::from_secret(config.jwt_secret.as_ref()),
                    &validation,
                ).map(|token_data| token_data.claims);

                // Check if decoding was successful
                if let Ok(token_data) = decoded_token {
                    // Extract the user ID from the token's claims
                    let user_id = match token_data.get("sub") {
                        Some(user_id) => user_id.to_string(),
                        None => return Outcome::Failure((Status::Unauthorized, JwtGuardError::InvalidToken)),
                    };

                    // Continue to the next middleware or request handler, with the user ID as the data
                    return Outcome::Success(JwtGuard { user_id });
                } else {
                    return Outcome::Failure((Status::Unauthorized, JwtGuardError::InvalidToken));
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, JwtGuardError::MissingToken))
    }
}
