use std::{error::Error, process::exit};

use actix_web::{dev::ServiceRequest, web::Data};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, TokenData, Validation, crypto::JwtVerifier};
use serde::{Deserialize, Serialize};

use crate::SiteData;

pub const ALGORITHM: Algorithm = Algorithm::EdDSA;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64
}

pub async fn check_bearer_jwt(
    req: ServiceRequest, 
    token: BearerAuth
) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    let data = req.app_data::<SiteData>().unwrap();
    match jsonwebtoken::decode::<Claims>(
        token.token(), 
        &data.decoding_key, 
        &Validation { 
            validate_exp: true,
            ..Validation::new(ALGORITHM)
        }
    ) {
        Ok(TokenData { header, claims }) => {
            
        },
        Err(rejection) => {}
    }
    Ok(req)
}


pub fn generate_token(sub: String, encoding_key: EncodingKey) -> Result<!, Box<dyn Error>> {
    let claim = Claims { sub, iat: Utc::now() };
    let token = jsonwebtoken::encode(
        &Header::new(ALGORITHM), 
        &claim, 
        &encoding_key
    )?;
    
    println!("{token}");
    exit(0)
}