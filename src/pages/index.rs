use actix_web::get;
use cached::once;
use maud::Markup;
use maud::Render;
use maud::attrs;
use maud::components::Component;
use maud::components::NoBody;
#[allow(unused)]
use maud::html;
use maud::write_html;

use crate::layout::*;
use crate::components::*;

const LAYOUT: GridLayout = grid! {
    _ => [
        links main main,
        links frens frens
    ];
    700 => [
        links main,
        links frens
    ];
    530 => [
        main,
        links,
        frens
    ]
};

#[once]
pub async fn index() -> Markup {
    html! {
        Page title="invoking my kitty" route="/" .. {
            Grid scope=(scope!()) layout=(LAYOUT) .. {
                Area label="links" .. {
                    .box {
                        ul.icon-list.fancy {
                            IconLi icon=(Icon::T) fg-force="lavender" .. {
                                a href="https://serenovv.tumblr.com" { "Tumblr" }
                            }
                            IconLi icon=(Icon::Photo) fg-force="peach" .. {
                                a href="https://www.instagram.com/stupidcatcreacher/" { "Instagram" }
                            }
                            IconLi icon=(Icon::Music) fg-force="red" .. {
                                a href="https://music.apple.com/profile/sery" { "Apple Music" }
                            }
                            IconLi icon=(Icon::SourceCode) fg-force="sapphire" .. {
                                a href="https://codeberg.org/sery" { "Codebergs" }
                            }
                            IconLi icon=(Icon::Cat) fg-force="text" .. {
                                a href="https://github.com/invokekitty" { "Github" }
                            }
                        }
                    }
                }
                Area label="main" .. {
                    //.box {
                        h2 { "sery"(NBSP)(NBSP)"•"(NBSP)(NBSP)"they/she" }
                        p {
                            r#"Haii im sery, dumbass cat that codes and enjoys music alot.
                               You can find projects and random shit i say here :3"#
                        }
                        p {
                            r#"Im an enby cat that lives somewhere in Europe (wont tell u where :3). I like coding,
                               mostly work on minecraft mods and other jvm-related stuff in kotlin, and pretty much anything
                               else i ever feel like doing in rust (including "# a href="https://codeberg.org/sery/webbed-site" { "this site" } r#"!)"#
                        }
                        p {
                            r#"Uhmmm, i use linux btw :3. Currently using "# a href="https://chimera-linux.org" { "Chimera Linux" } r#"
                               and "# a href="https://cachyos.org" { "CachyOS" }
                        }
                    //}
                }
                Area label="frens" .. {
                    Friendlink href="#" tooltip="Add me to your site :3" img="/assets/banners/me.webp" alt="Me";
                    Friendlink href="https://kibbyc.at" tooltip="skeedge" img="/assets/banners/riley.webp" alt="my cute catburger";
                    Friendlink href="https://mayaqq.dev" img="https://mayaqq.dev/assets/88x31/me.gif" alt="estrogen puppy" ..;
                    Friendlink href="https://resolv.moe" img="https://resolv.moe/friends/resolv.png?v2" alt="Maybe" ..;
                    Friendlink href="https://dynamicdispat.ch" img="https://dynamicdispat.ch/assets/img/button.png" alt="dyn is cute and gay" ..;
                    Friendlink href="https://awruff.codeberg.page" img="https://awruff.codeberg.page/assets/88x31.png" alt="shinyy ouppy :3" ..;
                }
            }

        }
    }
}

struct Friendlink<'a> {
    href: &'a str,
    img: &'a str,
    alt: &'a str,
    tooltip: Option<&'a str> = None
}

impl Component for Friendlink<'_> {
    type Body = NoBody;
    fn render_to(self, _body: NoBody, buffer: &mut String) {
        write_html! { buffer =>
            a.friendlink href=(self.href) data-tooltip=[self.tooltip] {
                img src=(self.img) alt=(self.alt);
            }
        }
    }
}
