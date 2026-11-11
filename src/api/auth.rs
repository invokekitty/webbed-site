use std::{error::Error, process::exit, time::Duration};

use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, query, query_as, sqlite::SqliteError};

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
        Ok(eeg) if let Ok(true) = enter_seerg(&data.database, &eeg.claims.sub, eeg.claims.iat).await => {
            eprintln!("{} is in !!!", &eeg.claims.sub);
            Ok(req)
        },
        Ok(unknown_eeg) => {
            eprintln!("{} was denied access because i have no idea who they are", &unknown_eeg.claims.sub);
            Err((ErrorUnauthorized("Who the hell are you"), req))
        }
        Err(rejection) => {
            eprintln!("Someone tried to use an invalid token: '{}': {rejection}", token.token());
            Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
        }
    }
}

// TODO: Background or something
async fn enter_seerg(db: &SqlitePool, seerg: &str, iat: u64) -> Result<bool, sqlx::Error> {
    let last_used = query_as::<_, (Option<DateTime<Utc>>,)>("SELECT last_use FROM seergs WHERE name = $1 AND created = $2")
        .bind(seerg)
        .bind(iat as i64)
        .fetch_optional(db)
        .await?;

    if let Some((lasteeg,)) = last_used {
        let now = Utc::now();
        match lasteeg {
            Some(date) => eprintln!("New access by '{seerg}' on {now}. Last access on {date}"),
            None => eprintln!("First access by '{seerg}' on {now}")
        }
        query("UPDATE seergs SET last_use = $1")
            .bind(now)
            .execute(db)
            .await?;
    }
    
    Ok(matches!(last_used, Some(_)))
}

pub async fn generate_token(db: &SqlitePool, sub: String, encoding_key: EncodingKey) -> Result<!, Box<dyn Error>> {
    
    let iat = jsonwebtoken::get_current_timestamp();
    let claim = Claims { sub: sub.clone(), iat, exp: iat + Duration::from_days(360).as_secs() };
    let token = jsonwebtoken::encode(
        &Header::new(ALGORITHM),
        &claim,
        &encoding_key
    )?;

    query("INSERT INTO seergs (name, created) VALUES ($1, $2)")
        .bind(&sub)
        .bind(iat as i64)
        .execute(db)
        .await?;
    
    println!("{token}");
    exit(0)
}

pub async fn reject_token(db: &SqlitePool, sub: String) -> Result<!, Box<dyn Error>> {
    query("DELETE FROM seergs WHERE name = $1")
        .bind(&sub)
        .execute(db)
        .await?;

    println!("Rejected token for '{sub}'");
    exit(0)
}