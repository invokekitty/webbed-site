use actix_web::{HttpRequest, Responder, Route};
use actix_web::web::{get, ServiceConfig};
use maud::{html, Markup};
use crate::layout::Page;

mod index;
mod posts;
mod projects;
mod servers;

pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "me", route: "/", builder: |route| route.to(index::index), .. },
    PageInfo { title: "posts", route: "/posts/", builder: |route| route.to(posts::posts), additional: Some(posts::configure) },
    PageInfo { title: "projects", route: "/projects", builder: |route| route.to(projects::projects), .. },
    PageInfo { title: "servers", route: "/servers", builder: |route| route.to(servers::servers), .. }
];

pub fn configure(config: &mut ServiceConfig) {
    for page in PAGES {
        config.route(page.route, (page.builder)(get()));
    }
}

pub struct PageInfo {
    pub title: &'static str,
    pub route: &'static str,
    builder: fn(Route) -> Route,
    additional: Option<fn(&mut ServiceConfig)> = None
}

pub async fn _404() -> Markup {
    html! {
        Page title="404 Error" route="" custom-container-class="container404" {
            .box404 {
                h1.text404 { "404!" }
                h3 { "somrawne got lost :3" }
                hr;
                p { "get back on track you " b { "cat" } }
            }
        }
    }
}