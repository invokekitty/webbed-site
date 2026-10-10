use actix_web::{HttpResponse, Responder, get, http::header::ContentType};


#[get("/styles/style.css")]
pub async fn style() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(grass::include!("./src/styles/main.scss"))
}
