use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, get, http::header::{CacheControl, CacheDirective::MaxAge}, web::{self, Data}};
use actix_web::web::ServiceConfig;
use cached::concurrent_cached;
use maud::{Markup, PreEscaped, Render, components::{Component, NoBody}, html, write_html};

use crate::{SiteData, api::blog::PostData, layout::Page};
use crate::api::blog;

pub async fn posts(data: Data<SiteData>) -> Result<impl Responder, actix_web::Error> {
    let posts = blog::get_all_posts(&data.database)
        .await
        .map_err(ErrorInternalServerError)?;
    
    let markup = html! {
        Page title="posts" route="/posts/" .. {
            
        }
    };

    Ok(HttpResponse::Ok()
        .append_header(CacheControl(vec![MaxAge(300)]))
        .body(markup))
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(get_post)
        .service(web::redirect("/posts", "/posts/"));
}

#[get("/posts/{id}")]
pub async fn get_post(id: web::Path<u64>, data: Data<SiteData>) -> impl Responder {
    let post = blog::fetch_post(&data.database, *id).await;
    html! {
        Page title=(format!("{id}").as_str()) route="/posts/" .. {
            .box {
                @match &post {
                    Ok(Some(post)) => {
                        Post info=(post);
                    },
                    Ok(None) => {
                        "This post doesn't exist"
                    }
                    Err(error) => {
                        "There was an error displying the post:"(error)
                    }
                }
            }
        }
    }
}

struct Post<'a> {
    info: &'a PostData
}

impl Component for Post<'_> {
    type Body = NoBody;
    fn render_to(self, _body: NoBody, buffer: &mut String) {
        let body = PreEscaped(&self.info.content.0);
        write_html! { buffer =>
            h1 { (self.info.title) }
            (body)
        }
    }
}
