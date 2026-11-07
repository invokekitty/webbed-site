use actix_web::{get, HttpRequest, Responder};


pub async fn servers(req: HttpRequest) -> impl Responder {
    req.query_string().to_string()
}
