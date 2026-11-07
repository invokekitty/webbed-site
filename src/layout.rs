use maud::{ComponentBody, DOCTYPE, PreEscaped, Render, write_html};

use crate::pages::PageInfo;
use crate::{components::Icon, pages::PAGES};

pub struct Page<'a> {
    pub title: &'a str,
    pub route: &'a str,
    pub body: ComponentBody<'a>,
    pub custom_container_class: Option<&'a str> = None,
    pub pages: &'a [PageInfo] = PAGES
}

impl Render for Page<'_> {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            (DOCTYPE)
            html lang="en" {
                head {
                    title { (self.title) }
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width,initial-scale=1.0";
                    link rel="icon" href="/assets/favicon.ico";
                    link rel="stylesheet" href="/styles/style.css";
                    link rel="stylesheet" href="/api/meow";
                }
                body dir="ltr" {
                    Background;
                    Navbar pages=(self.pages) route=(self.route);
                    main.content {
                        div class=(if let Some(c) = self.custom_container_class { c } else { "container" }) {
                            (self.body)
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
}

pub struct Background;
impl Render for Background {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            img.bg-img src="/assets/images/bg.webp" alt="background" aria-hidden="true";
        }
    }
}

pub struct Navbar<'a> {
    pub pages: &'a [PageInfo] = PAGES,
    pub route: &'a str
}

impl Render for Navbar<'_> {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            .navbar {
                input #nav-check type="checkbox";
                label.navbar-text for="nav-check" {
                    span style="font-size:75%" { (Icon::Menu) }
                }
                nav.nav-menu style=(format_args!("--height: {}px", 53.33 * self.pages.len() as f32)) {
                    @for page in self.pages {
                        a.nav-item .active[page.route == self.route] href=(page.route) {
                            (page.title)
                        }
                    }
                }
            }
        }
    }
}
