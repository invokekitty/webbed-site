use actix_web::{get, HttpResponse, Responder};
use maud::html;

use crate::layout::Page;

#[get("/posts/")]
pub async fn posts() -> impl Responder {
    let markup = html! {
        Page title="posts" route="/posts/" {
            
        }
    };

    HttpResponse::Ok()
        .body(markup)
}

#[get("/posts")]
pub async fn posts_redirect() -> impl Responder {
    HttpResponse::MovedPermanently()
        .append_header(("Location", "/posts/"))
        .body("")
}

