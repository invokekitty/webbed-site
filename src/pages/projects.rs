use actix_web::get;
use cached::once;
use maud::ComponentBody;
use maud::Markup;
use maud::Render;
use maud::html;
use maud::write_html;

use Platform::*;
use crate::components::*;
use crate::layout::Page;

#[get("/projects")]
#[once]
pub async fn projects() -> Markup {
    html! {
        Page title="invoking the projects" route="/projects" {
            h1 { "Projects i made" }
            hr.thicc;
            Project #kritter links=(&[(Codeberg, "sery/kritter")][..]) {
                h2 { "kritter" }
                p { "Minecraft multiplatform library with a declarative style and a more kotlin take on things. Soon to be merged with maya's cynosure" }
            }
            Project #nullbus links=(&[(Codeberg, "sery/nullbus")][..]) {
                h2 { "nullbus" }
                p { "Event bus born out of delusions and insanity with equally delusional and insane benchmarks" }
            }
            Project #nullevt links=(&[(Codeberg, "sery/nullevt")][..]) {
                h2 { "nullevt" }
                p { "Event system with a different architecture that is even faster than nullbus (and thread safe by default !!)" }
            }
            Project #serycade links=(&[(Codeberg, "sery/serycade"), (AUR, "serycade")][..]) {
                h2 { "serycade" }
                p { "Silly tui games i made in kotlin/native bcs riley is cute and am gradually rustifying" }
            }
            Project #ktindy links=(&[(Codeberg, "sery/ktindy")][..]) {
                h2 { "KtIndy" }
                p {
                    "A kotlin compiler plugin + runtime that makes kotlin use invokedynamic in more places where it makes sense. "
                    "Currently this includes annotation constructors and typeOf calls, but function and property reference are also planned"
                }
                p { "Featuring a bit of evil jvm fuckery" }
            }
            Project #website links=(&[(Codeberg, "sery/webbed-site"), (Site, "https://kitty.kisse.su")][..]) {
                h2 { "this website :3" }
                p {
                    "Yea this fkin website thats about it, catburger convinced me to switch to astro and helped me abit, and now i rewrote it in rust, "
                    "because native tastes better than js and furry code tastes better than claude code."
                }
            }
            Project #snowed-in deprecated links=(&[(Codeberg, "sery/snowy-stuff"), (Modrinth, "snowed-in")][..]) {
                h2 { "snowed in" }
                p { "Snowloggin mod i made, it sucks dont use it, also no longer maintained until i rewrite it" }
            }
            h1 { "Projects i contribute to" }
            hr.thicc;
            Project #estrogen links=(&[(Github, "MayaqqDev/Estrogen"), (Modrinth, "estrogen"), (CurseForge, "estrogen")][..]) {
                h2 { "estrogen mod" }
                p { "A mod about estrogen i think" }
            }
            Project #cynosure links=(&[(Github, "MayaqqDev/Cynosure"), (Modrinth, "cynosure"), (CurseForge, "cynosure")][..]) {
                h2 { "cynosure" }
                p { "cool big modding library thats gonna be merged with kritter" }
            }
        }
    }
}

struct Project<'a> {
    id: &'a str,
    links: &'a [(Platform, &'a str)],
    deprecated: bool = false,
    body: Option<ComponentBody<'a>> = None
}

impl Render for Project<'_> {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            #(self.id) .project {
                @if self.deprecated {
                    span.deprecated-marker { Icon::Warning; (NBSP)"Deprecated" }
                }
                @if let Some(ref body) = self.body {
                    (body)
                }
                span.fancy {
                    @for (index, (platform, link)) in self.links.iter().enumerate() {
                        @let (icon, name, color, linkprefix) = platform.data();
                        @let href = format!("{linkprefix}{link}");
                        span {
                            @if index != 0 { span.delim { "•" } }
                            a href=(href) data-fg=(color) { (icon)(NBSP)(name) }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Platform {
    Codeberg,
    Github,
    Modrinth,
    CurseForge,
    AUR,
    Site
}

impl Platform {
    pub fn data(&self) -> (Icon, &'static str, &'static str, &'static str) {
        match self {
            Platform::Codeberg => (Icon::SourceCode, "Codeberg", "sapphire", "https://codeberg.org/"),
            Platform::Github => (Icon::Cat, "Github", "text", "https://github.com/"),
            Platform::Modrinth => (Icon::Cog, "Modrinth", "green", "https://mods.gay/"),
            Platform::CurseForge => (Icon::ShoppingCartIThink, "CurseForge", "peach", "https://curseforge.org/minecraft/mc-mods/"),
            Platform::AUR => (Icon::IUseArchBtw, "AUR", "blue", "https://aur.archlinux.org/packages/"),
            Platform::Site => (Icon::Safari, "Website", "sky", "")
        }
    }
}
