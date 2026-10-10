use actix_web::{Responder, get};
use maud::html;

use crate::{components::grid, layout::{PageInfo, page}};


pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "Home", route: "/" },
    PageInfo { title: "Pronouns n stuff", route: "/me" },
    PageInfo { title: "Projects", route: "/projects" },
    PageInfo { title: "Servers", route: "/servers" }
];


#[get("/")]
pub async fn index() -> impl Responder {
    page("invoking my kitty", "/", grid! {
        autocols;
        [ "links main main", "links frens frens" ];
        @530>[ "main", "links", "frens" ];
        links: {
            .box {

            }
        }
        main: {
            .box {

            }
        }
        frens: {
            .box {

            }
        }
    })
}
