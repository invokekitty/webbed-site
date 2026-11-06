use actix_web::web::ServiceConfig;
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

pub fn pages(config: &mut ServiceConfig) {
    let x = &index::index;


    config.service(index::index)
        .service(posts::posts)
        .service(posts::posts_redirect)
        .service(projects::projects)
        .service(servers::servers);
}
