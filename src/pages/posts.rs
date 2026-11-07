use actix_web::{HttpResponse, Responder, get, web::{self, Data}};
use cached::concurrent_cached;
use maud::{Markup, Render, html, write_html};

use crate::{SiteData, api::blog::PostData, layout::Page};

#[get("/posts/")]
pub async fn posts() -> impl Responder {
    let markup = html! {
        Page title="posts" route="/posts/" {
            
        }
    };

    HttpResponse::Ok()
        .body(markup)
}

// #[concurrent_cached(ttl = "Duration::from_secs(30)")]
// pub async fn posts_body(posts) -> Markup {
//     
// }

#[get("/posts/{id}")] 
pub async fn get_post(id: web::Path<u64>, data: Data<SiteData>) -> impl Responder {
    html! {
        Page title=(format!("{id}").as_str()) route="/posts/" {
            
        }
    }
}

#[get("/posts")]
pub async fn posts_redirect() -> impl Responder {
    HttpResponse::MovedPermanently()
        .append_header(("Location", "/posts/"))
        .body("Moved to /posts/")
}

struct Post {
    info: PostData,
    trunc_body: usize = 65
}

impl Render for Post {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            
        }
    }
}