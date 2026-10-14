use actix_web::{Responder, get};
use maud::html;

use crate::layout::*;
use crate::components::*;

#[get("/projects")]
pub async fn get() -> impl Responder {
    page("kittyprojects", "/projects", html! {
        h1 { "Projects i made" }
        hr.thicc;
        (project ("kritter", &[(Platform::Codeberg, "sery/kritter")], false, html! {
            h2 { "kritter" }
            p { "Minecraft multiplatform library with a declarative style and a more kotlin take on things. Soon to be merged with maya's cynosure" }
        }))
        (project ("nullbus", &[(Platform::Codeberg, "sery/nullbus")], false, html! {
            h2 { "nullbus" }
            p { "Event bus born out of delusions and insanity with equally delusional and insane benchmarks" }
        }))
        (project ("nullevt", &[(Platform::Codeberg, "sery/nullevt")], false, html! {
            h2 { "nullevt" }
            p { "Event system with a different architecture that is even faster than nullbus (and thread safe by default !!)" }
        }))
        (project ("serycade", &[(Platform::Codeberg, "sery/serycade"), (Platform::AUR, "serycade-git")], false, html! {
            h2 { "serycade" }
            p { "Silly tui games i made in kotlin/native bcs riley is cute and am gradually rustifying" }
        }))
        (project ("ktindy", &[(Platform::Codeberg, "sery/ktindy")], false, html! {
            h2 { "KtIndy" }
            p { r#"
                A kotlin compiler plugin + runtime that makes kotlin use invokedynamic in more places where it makes sense.
                Currently this includes annotation constructors and typeOf calls, but function and property reference are also planned
                "# }
            p { "Featuring a bit of evil jvm fuckery" }
        }))
        (project ("website", &[(Platform::Codeberg, "sery/webbed-site"), (Platform::Site, "https://kitty.kisse.su")], false, html! {
            h2 { "this website :3" }
            p {
                "Yea this fkin website thats about it, catburger convinced me to switch to astro and helped me abit, and now i rewrote it in rust, "
                "because native tastes better than js and furry code tastes better than claude code."
            }
        }))
        (project ("snowed-in", &[(Platform::Codeberg, "sery/snowy_stuff")], true, html! {
            h2 { "snowed in" }
            p { "Snowloggin mod i made, it sucks dont use it, also no longer maintained until i rewrite it" }
        }))
        h1 { "Projects i contributed to" }
        hr.thicc;
        (project ("estrogen", &[(Platform::Github, "MayaqqDev/Estrogen"), (Platform::Modrinth, "estrogen"), (Platform::CurseForge, "estrogen")], false, html! {
            h2 { "estrogen mod" }
            p { "A mod about estrogen i think" }
        }))
        (project ("cynosure", &[(Platform::Github, "MayaqqDev/Cynosure"), (Platform::Modrinth, "cynosure"), (Platform::CurseForge, "cynosure")], false, html! {
            h2 { "cynosure" }
            p { "A kotlin modding library that does alot, soon to be merged with kritter" }
        }))
    })
}
