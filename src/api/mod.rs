use actix_web::{post, Error, Responder};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::{Data, ReqData, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::query;
use crate::api::auth::TokenOwner;
use crate::SiteData;

pub mod blog;
pub mod auth;


pub fn configure(config: &mut ServiceConfig) {
    config
        .service(change_pfp)
        .configure(blog::web::configure);

}

#[post("/api/change-pfp", wrap = "HttpAuthentication::bearer(auth::check_bearer_jwt)")]
async fn change_pfp(site_data: Data<SiteData>, eeg: ReqData<TokenOwner>, body: Option<String>) -> Result<impl Responder, Error> {
    query("UPDATE seergs SET pfp = $2 WHERE name = $1")
        .bind::<&str>(eeg.as_ref())
        .bind(body)
        .execute(&site_data.database)
        .await.map_err(ErrorInternalServerError)?;
    
    Ok("Success :3")
}
