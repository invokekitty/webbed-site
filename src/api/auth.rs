use std::{error::Error, process::exit, time::Duration};

use actix_web::{dev::ServiceRequest, web::Data};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{Algorithm, EncodingKey, Header, Validation};
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
    let data = req.app_data::<Data<SiteData>>().unwrap();
    match jsonwebtoken::decode::<Claims>(
        token.token(), 
        &data.decoding_key, 
        &Validation { 
            validate_exp: true,
            ..Validation::new(ALGORITHM)
        }
    ) {
        Ok(eeg) => {
            eprintln!("{} is in !!!", &eeg.claims.sub);
            Ok(req)
        },
        Err(rejection) => {
            eprintln!("Someone tried to use an invalid token: '{}'", token.token());
            Err((actix_web::error::ErrorUnauthorized(rejection), req))
        } 
    }
}


pub fn generate_token(sub: String, encoding_key: EncodingKey) -> Result<!, Box<dyn Error>> {
    let iat = jsonwebtoken::get_current_timestamp();
    let claim = Claims { sub, iat, exp: iat + Duration::from_days(360).as_secs() };
    let token = jsonwebtoken::encode(
        &Header::new(ALGORITHM), 
        &claim, 
        &encoding_key
    )?;
    
    println!("{token}");
    exit(0)
}