use crate::layout::*;

pub mod index;
pub mod posts;
pub mod projects;
pub mod servers;

pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "me", route: "/" },
    PageInfo { title: "posts", route: "/posts" },
    PageInfo { title: "projects", route: "/projects" },
    PageInfo { title: "servers", route: "/servers" }
];
