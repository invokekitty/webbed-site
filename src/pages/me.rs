use std::sync::OnceLock;

use actix_web::{Responder, get};
use chrono::{DateTime, Utc};
use maud::html;

use crate::layout::page;
use crate::components::*;

pub static SEERGYBIRTH: OnceLock<DateTime<Utc>> = OnceLock::new();

#[get("/me")]
pub async fn get() -> impl Responder {
    let today = Utc::now();
    let seergyage = SEERGYBIRTH.get().and_then(|bd| today.years_since(*bd)).unwrap_or(14);

    page("invoking the pronouns", "/me", html! {
        .box {
            (grid! {
                autocols;
                [
                    "title title title title title title",
                    "desc desc desc flags flags flags",
                    "names names prns prns compl compl",
                    "legend legend legend legend legend legend"
                ];
                @700 [
                    "title title title",
                    "desc desc flags",
                    "names name names",
                    "prns prns prns",
                    "compl compl compl",
                    "legend legend legend"
                ];
                @530 [
                    "title",
                    "flags",
                    "desc",
                    "names",
                    "prns",
                    "compl",
                    "legend"
                ];
                title: {
                    h1.caption { "Serenity" }
                }
                desc: {
                    p {
                        r#"Im a girl in the sense that im not, if ur here u probably read the front page anyways u eeg"#
                    }
                    //br;br;
                    "Age: " b { (seergyage) }
                }
                names: {
                    h2 { "My Names :3" }
                    (iconlist! {
                        Heart [data-fg="pink"] { "serenity" }
                        Heart [data-fg="pink"] { "sery" }
                        Updoot [data-fg="text"] { "serena" }
                    })
                }
                prns: {
                    h2 { "My pronouns" }
                    (iconlist! {
                        Heart@[data-tooltip="Enby fuck"] [data-fg="pink"] { "they/them" }
                        Heart@[data-tooltip="im a  girl kind of"] [data-fg="pink"] { "she/her" }
                        Updoot@[data-tooltip="also a bit of a thing.."] [data-fg="text"] { "it/its" }
                    })
                }
                compl: {
                    h2 { "Things to refer to me with" }
                    (iconlist! {
                        Heart@[data-tooltip="LOIK thiS"] [data-fg="pink"] { "girl" }
                        Heart@[data-tooltip="MRRRAAAWWW"] [data-fg="pink"] { "cat" }
                        Updoot@[data-tooltip="i be a bit of a boi"] [data-fg="text"] { "boi" }
                        Group@[data-tooltip="only if ur mine <3"] [data-fg="red"] { "girlfriend" }
                        Group@[data-tooltip="u faggot"] [data-fg="red"] { "boyfriend" }
                        Group@[data-tooltip="in crime"] [data-fg="red"] { "partner" }
                    })
                }
                legend: {
                    ul.legend-content {
                        li { (Icon::Heart) "like this much" }
                        li { (Icon::Updoot) "oki with this" }
                        li { (Icon::Quoip) "only joking" }
                        li { (Icon::Group) "only if close" }
                    }
                }
            })
        }
    })
}
