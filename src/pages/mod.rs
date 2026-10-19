use crate::layout::*;

pub mod me;
pub mod index;
pub mod projects;
pub mod servers;

pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "Home", route: "/" },
    PageInfo { title: "Pronouns n stuff", route: "/me" },
    PageInfo { title: "Projects", route: "/projects" },
    PageInfo { title: "Servers", route: "/servers" }
];
