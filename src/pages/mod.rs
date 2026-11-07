use actix_web::web::{ServiceConfig, get};
use crate::layout::*;

mod index;
mod posts;
mod projects;
mod servers;

pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "me", route: "/" },
    PageInfo { title: "posts", route: "/posts/" },
    PageInfo { title: "projects", route: "/projects" },
    PageInfo { title: "servers", route: "/servers" }
];

pub fn configure(config: &mut ServiceConfig) {
    config.service(index::index)
        .service(posts::posts)
        .service(posts::posts_redirect)
        .service(projects::projects)
        .service(servers::servers);
}
