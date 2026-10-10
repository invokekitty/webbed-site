use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::{components::Icon, pages::PAGES};


pub fn page(title: &str, route: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { (title) }
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/favicon.ico";
                link rel="stylesheet" href="/styles/style.css";
                link rel="stylesheet" href="/api/meow";
            }
            body dir="ltr" {
                (background())
                (navbar(PAGES, route))
                main.content {
                    .container {
                        (content)
                    }
                }
                footer.footer {
                    .container-full.footertext {
                        small.fancy { "background by " a href="https://www.deviantart.com/pixeljeff" { "@pixeljeff" } }
                        small.fancy.visit-counter { (Icon::Goblin) }
                    }
                }
                (PreEscaped("<!--[if IE]>"))
                script src="/scripts/oneko-ie6.js" data-cat="/assets/images/oneko.gif" {}
                (PreEscaped(
                    r#"<![endif]-->
                    <!--[if !IE]><!-->"#
                ))
                script src="/assets/scripts/oneko.js" data-cat="/assets/images/oneko.gif" {}
            }
        }
    }
}

fn background() -> Markup {
    html! {
        img.bg-img src="/assets/images/bg.webp" alt="background";
    }
}

pub struct PageInfo { pub title: &'static str, pub route: &'static str }

fn navbar(pages: &[PageInfo], active_route: &str) -> Markup {
    html! {
        header.navbar {
            input #nav-check type="checkbox";
            label.navbar-text for="nav-check" {
                span style="font-size:75%" { (Icon::Menu) }
            }
            nav.nav-menu {
                @for page in pages {
                    a.nav-item .active[page.route == active_route] {
                        (page.title)
                    }
                }
            }
        }
    }
}
