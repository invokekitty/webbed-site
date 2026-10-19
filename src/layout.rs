use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::{components::Icon};


pub fn page(pages: &[PageInfo], title: &str, route: &str, content: Markup) -> Markup {
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
                (navbar(pages, route))
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
                script src="/scripts/oneko-ie6.js" data-cat="/images/oneko.gif" {}
                (PreEscaped(
                    r#"<![endif]-->
                    <!--[if !IE]><!-->"#
                ))
                script src="/scripts/oneko.js" data-cat="/images/oneko.gif" {}
                (PreEscaped("<!--[endif]><!-->"))
            }
        }
    }
}

fn background() -> Markup {
    html! {
        img.bg-img src="/images/bg.webp" alt="background";
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
            nav.nav-menu style=(format!("--height: {}px", 53.33 * pages.len() as f32)) {
                @for page in pages {
                    // TODO: Only check the part before '?' and '#'
                    a.nav-item .active[page.route == active_route] href=(page.route) {
                        (page.title)
                    }
                }
            }
        }
    }
}
