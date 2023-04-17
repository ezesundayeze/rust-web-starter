use std::env;
use jsonwebtoken::Algorithm;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_algorithm: Algorithm,
    pub database_name: String
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Config {
            database_url: env::var("MONGODB_URI").expect("MONGODB_URI not found in environment"),
            database_name: env::var("DB_NAME").expect("DB_NAME not found in environment"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not found in environment"),
            jwt_algorithm: match env::var("JWT_ALGORITHM").expect("JWT_ALGORITHM not found in environment").as_str() {
                "HS256" => Algorithm::HS256,
                "HS384" => Algorithm::HS384,
                "HS512" => Algorithm::HS512,
                "RS256" => Algorithm::RS256,
                "RS384" => Algorithm::RS384,
                "RS512" => Algorithm::RS512,
                _ => panic!("Invalid JWT algorithm"),
            },
        }
    }
}
