use actix_web::{HttpRequest, Responder, get};


#[get("/servers")]
pub async fn get(req: HttpRequest) -> impl Responder {
    req.query_string().to_string()
}
