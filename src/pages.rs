use actix_web::{Responder, get};
use maud::html;

use crate::{components::{friendlink, grid, iconli}, layout::{PageInfo, page}};


pub const PAGES: &'static [PageInfo] = &[
    PageInfo { title: "Home", route: "/" },
    PageInfo { title: "Pronouns n stuff", route: "/me" },
    PageInfo { title: "Projects", route: "/projects" },
    PageInfo { title: "Servers", route: "/servers" }
];


#[get("/")]
pub async fn index() -> impl Responder {
    page("invoking my kitty", "/",
        grid! { autocols; [ "links main main", "links frens frens" ]; @530>[ "main", "links", "frens" ];
            links: {
                .box {
                    ul."icon-list" {
                        (iconli! [ Safari [data-fg-force="lavender"] { a href="https://serenovv.tumblr.com" { "Tumblr" } } ])
                        (iconli! [ Photo [data-fg-force="peach"] { a href="https://www.instagram.com/stupidcatcreacher/" { "Instagram" } } ])
                        (iconli! [ Music [data-fg-force="red"] { a href="https://music.apple.com/profile/sery" { "Apple Music" } } ])
                        (iconli! [ SourceCode [data-fg-force="sapphire"] { a href="https://codeberg.org/sery" { "Codebergs" } } ])
                    }
                }
            }
            main: {
                .box {
                    h2 { "Welcome to sery's webbed site :3" }
                    p {
                        r#"Haii im sery, dumbass cat that codes and enjoys music alot.
                        You can find projects and random shit i say here :3"#
                        br;br;
                        r#"
                        I use chimera linux and cachyos :3
                        "#
                    }
                }
            }
            frens: {
                (friendlink ("#", Some("Add me to your site :3"), "/assets/banners/me.webp", "Me"))
                (friendlink ("https://kibbyc.at", Some("skeedge"), "/assets/banners/riley.webp", "my cute catburger"))
                (friendlink ("https://mayaqq.dev", None, "https://mayaqq.dev/assets/88x31/me.gif", "estrogen puppy"))
                (friendlink ("https://resolv.moe", None, "https://resolv.moe/friends/resolv.png?v2", "Maybe"))
                (friendlink ("https://dynamicdispat.ch", None, "https://dynamicdispat.ch/assets/img/button.png", "dyn is cute and gay"))
                (friendlink ("https://awruff.dog", None, "https://awruff.dog/assets/88x31.png", "shinyyy ouppy :3"))
            }
        }
    )
}
