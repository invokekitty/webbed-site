use actix_web::{error::ErrorInternalServerError, get, http::header::{CacheControl, CacheDirective::MaxAge}, web::{self, Data}, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::Header;
use actix_web::web::{ServiceConfig};
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
            // .posts-sidebar {
            //     
            // }
            .posts {
                @for post in &posts {
                    Post info=(post);
                }
            }
        }
    };

    Ok(HttpResponse::Ok()
        .append_header(CacheControl(vec![MaxAge(300)]))
        .body(markup))
}

pub fn configure(config: &mut ServiceConfig) {
    config //.service(get_post)
        .service(web::redirect("/posts", "/posts/"));
}


struct Post<'a> {
    info: &'a PostData
}

impl Component for Post<'_> {
    type Body = NoBody;
    fn render_to(self, _body: NoBody, buffer: &mut String) {
        let mut ellipse: bool = true;
        let summary = PreEscaped(self.info
            .summary
            .as_ref()
            .map(String::as_str)
            .unwrap_or_else(|| &self.info.content.0
                .trunc_at_word(70)
                .unwrap_or_else(|| { ellipse = false; &self.info.content.0 }))
        );
        let body = PreEscaped(&self.info.content.0);
        let authoer = &self.info.author;

        write_html! { buffer =>
            #(self.info.id().unwrap()) .post {
                h5 { img.post-pfp .inline-icon src=(format_args!("/assets/images/{authoer}.webp")); " "(authoer) }
                small.post-date { (self.info.posted_on) }
                (summary)@if ellipse { "…" }
                dialog.expanded-content {
                    (body)
                }
            }
        }
    }
}

const SEPARATORS: &[char] = &[' ', '.', ',', '!', '?', '(', ')', '"', '\n', '{', '}', '[', ']', ';', ':', '\t']; // " (){}[].,;:"


trait TruncWordsExt {
    fn trunc_at_word(&self, count: usize) -> Option<&str>;
}

impl TruncWordsExt for str {
    fn trunc_at_word(&self, mut chars: usize) -> Option<&str> {
        let mut last_word: usize = 0;
        let mut iter = self.char_indices();

        while let Some((index, chr)) = iter.next() && chars > 0 {
            if SEPARATORS.contains(&chr) { last_word = index; }
            chars -= 1;
        }

        if chars > 0 { None } else { Some(&self[0..last_word]) }
    }
}