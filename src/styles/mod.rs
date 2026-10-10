use actix_web::{Responder, get};


#[get("/styles/style.scss")]
pub async fn style() -> impl Responder {
    grass::include!("./src/styles/main.scss")
}
