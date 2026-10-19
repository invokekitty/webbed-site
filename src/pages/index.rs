use actix_web::{Responder, get};
#[allow(unused)]
use maud::html;

use crate::layout::*;
use crate::components::*;
use crate::pages::PAGES;


#[get("/")]
pub async fn get() -> impl Responder {
    page(PAGES, "invoking my kitty", "/",
        grid! { [ "links main main", "links frens frens" ]; @530 [ "main", "links", "frens" ];
            links: {
                .box {
                    ul."icon-list".fancy {
                        (iconli! ( T [data-fg-force="lavender"] { a href="https://serenovv.tumblr.com" { "Tumblr" } } ))
                        (iconli! ( Photo [data-fg-force="peach"] { a href="https://www.instagram.com/stupidcatcreacher/" { "Instagram" } } ))
                        (iconli! ( Music [data-fg-force="red"] { a href="https://music.apple.com/profile/sery" { "Apple Music" } } ))
                        (iconli! ( SourceCode [data-fg-force="sapphire"] { a href="https://codeberg.org/sery" { "Codebergs" } } ))
                    }
                }
            }
            main: {
                .box {
                    h2 { "Welcome to sery's webbed site :3" }
                    p {
                        r#"Haii im sery, dumbass cat that codes and enjoys music alot.
                        You can find projects and random shit i say here :3"#
                    }
                    p {
                        r#"
                        Im an enby cat that lives somewhere in Europe (wont tell u where :3). I like coding,
                        mostly work on minecraft mods and other jvm-related stuff in kotlin, and pretty much anything
                        else i ever feel like doing in rust (incuding "# a href="https://codeberg.org/sery/webbed-site" { "this site" } r#"!)

                        "#
                    }
                }
            }
            frens: {
                (friendlink ("#", Some("Add me to your site :3"), "/banners/me.webp", "Me"))
                (friendlink ("https://kibbyc.at", Some("skeedge"), "/banners/riley.webp", "my cute catburger"))
                (friendlink ("https://mayaqq.dev", None, "https://mayaqq.dev/assets/88x31/me.gif", "estrogen puppy"))
                (friendlink ("https://resolv.moe", None, "https://resolv.moe/friends/resolv.png?v2", "Maybe"))
                (friendlink ("https://dynamicdispat.ch", None, "https://dynamicdispat.ch/assets/img/button.png", "dyn is cute and gay"))
                (friendlink ("https://awruff.dog", None, "https://awruff.dog/assets/88x31.png", "shinyyy ouppy :3"))
            }
        }
    )
}
