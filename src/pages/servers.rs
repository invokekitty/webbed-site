use actix_web::{get, HttpRequest, Responder};


#[get("/servers")]
pub async fn servers(req: HttpRequest) -> impl Responder {
    req.query_string().to_string()
}
