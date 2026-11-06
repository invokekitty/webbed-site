use actix_web::{Responder, get};
use maud::html;

use crate::layout::Page;

#[get("/posts")]
pub async fn get() -> impl Responder {
    html! {
        Page title="posts" route="/posts" {
            
        }
    }
}