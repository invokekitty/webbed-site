use std::{sync::OnceLock, time::SystemTime};

use actix_web::{Responder, get};
use chrono::{DateTime, Local, TimeZone, Utc};
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
        grid! { [ "links main main", "links frens frens" ]; @530 [ "main", "links", "frens" ];
            links: {
                .box {
                    ul."icon-list".fancy {
                        (iconli! ( Safari [data-fg-force="lavender"] { a href="https://serenovv.tumblr.com" { "Tumblr" } } ))
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
                        br;br;
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

pub static SEERGYBIRTH: OnceLock<DateTime<Utc>> = OnceLock::new();

#[get("/me")]
pub async fn me() -> impl Responder {
    let today = Utc::now();
    let seergyage = SEERGYBIRTH.get().and_then(|bd| today.years_since(*bd)).unwrap_or(14);

    page("invoking the pronouns", "/me", html! {
        .box {
            (grid! {
                [
                    "title title title title title title",
                    "desc desc desc flags flags flags",
                    "names names prns prns compl compl",
                    "legend legend legend legend legend legend"
                ];
                title: {
                    h1.caption { "Serenity" }
                }
                desc: {
                    r#"Im a girl in the sense that im not, if ur here u probably read the front page anyways u eeg"#
                    br;br;
                    "Age: " b { (seergyage) }
                }
                names: {
                    h2 { "My Names :3" }
                    ul.icon-list {
                        (iconli!(Heart [data-fg="pink"] { "Serenity" }))
                        (iconli!(Heart [data-fg="pink"] { "Sery" }))
                        (iconli!(Updoot [data-fg="text"] { "Serena" }))
                    }
                }
                prns: {
                    h2 { "My pronouns" }
                    ul.icon-list {
                        (iconli!(Heart [data-fg="pink"] { "they/them" }))
                        (iconli!(Heart [data-fg="pink"] { "she/her" }))
                        (iconli!(Updoot [data-fg="text"] { "it/its" }))
                    }
                }
                compl: {
                    h2 { "Things to refer to me with" }
                    ul.icon-list {

                    }
                }
            })
        }
    })
}
