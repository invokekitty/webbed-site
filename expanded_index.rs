pub mod index {
    use actix_web::get;
    use cached::once;
    use maud::Markup;
    use maud::Render;
    #[allow(unused)]
    use maud::html;
    use maud::write_html;
    use crate::layout::*;
    use crate::components::*;
    #[allow(non_camel_case_types)]
    pub struct get;
    impl ::actix_web::dev::HttpServiceFactory for get {
        fn register(self, __config: &mut actix_web::dev::AppService) {
            ///Cached static for the [`get`] function.
            pub static GET: ::std::sync::LazyLock<
                ::cached::async_sync::RwLock<Option<Markup>>,
            > = ::std::sync::LazyLock::new(|| ::cached::async_sync::RwLock::new(None));
            ///This is a cached function that uses the [`GET`] cached static.
            pub async fn get() -> Markup {
                {
                    let __cached_cached = GET.read().await;
                    if true {
                        if let Some(__cached_result) = &*__cached_cached {
                            return <Markup as ::std::clone::Clone>::clone(
                                __cached_result,
                            );
                        }
                    }
                }
                async fn get_no_cache() -> Markup {
                    let layout = {
                        crate::components::GridLayout::new(
                            crate::components::GridDefinition(
                                None,
                                &["links main main ", "links frens frens "],
                            ),
                            &[
                                crate::components::GridDefinition(
                                    Some(700),
                                    &["links main ", "links frens "],
                                ),
                                crate::components::GridDefinition(
                                    Some(530),
                                    &["main ", "links ", "frens "],
                                ),
                            ],
                        )
                    };
                    {
                        extern crate alloc;
                        extern crate maud;
                        let mut __maud_output = alloc::string::String::with_capacity(
                            2977usize,
                        );
                        {
                            use ::maud::macro_private::*;
                            match ChooseRenderOrDisplay(
                                &(Page {
                                    title: "invoking my kitty".into(),
                                    route: "/".into(),
                                    body: maud::macro_private::new_component_body(
                                            &|__maud_buffer: &mut String| {
                                                extern crate alloc;
                                                extern crate maud;
                                                let mut __maud_output: &mut alloc::string::String = {
                                                    __maud_buffer
                                                };
                                                {
                                                    use ::maud::macro_private::*;
                                                    match ChooseRenderOrDisplay(
                                                        &(Grid {
                                                            scope: ("seergysite::pages::index-29-25").into(),
                                                            layout: (layout).into(),
                                                            body: maud::macro_private::new_component_body(
                                                                    &|__maud_buffer: &mut String| {
                                                                        extern crate alloc;
                                                                        extern crate maud;
                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                            __maud_buffer
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "links".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                __maud_output
                                                                                                    .push_str(
                                                                                                        "<div class=\"box\"><ul class=\"icon-list fancy\">",
                                                                                                    );
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::T).into(),
                                                                                                            fg_force: "lavender".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://serenovv.tumblr.com\">Tumblr</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Photo).into(),
                                                                                                            fg_force: "peach".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://www.instagram.com/stupidcatcreacher/\">Instagram</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Music).into(),
                                                                                                            fg_force: "red".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://music.apple.com/profile/sery\">Apple Music</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::SourceCode).into(),
                                                                                                            fg_force: "sapphire".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://codeberg.org/sery\">Codebergs</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Cat).into(),
                                                                                                            fg_force: "text".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://github.com/invokekitty\">Github</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                __maud_output.push_str("</ul></div>");
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "main".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                __maud_output
                                                                                                    .push_str(
                                                                                                        "<div class=\"box\"><h2>Welcome to sery's webbed site :3</h2><p>Haii im sery, dumbass cat that codes and enjoys music alot.\n                               You can find projects and random shit i say here :3</p><p>Im an enby cat that lives somewhere in Europe (wont tell u where :3). I like coding,\n                               mostly work on minecraft mods and other jvm-related stuff in kotlin, and pretty much anything\n                               else i ever feel like doing in rust (including <a href=\"https://codeberg.org/sery/webbed-site\">this site</a>!)</p><p>Uhmmm, i use linux btw :3. Currently using <a href=\"https://chimera-linux.org\">Chimera Linux</a>\n                               and <a href=\"https://cachyos.org\">CachyOS</a></p></div>",
                                                                                                    );
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "frens".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "#".into(),
                                                                                                            tooltip: "Add me to your site :3".into(),
                                                                                                            img: "/banners/me.webp".into(),
                                                                                                            alt: "Me".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://kibbyc.at".into(),
                                                                                                            tooltip: "skeedge".into(),
                                                                                                            img: "/banners/riley.webp".into(),
                                                                                                            alt: "my cute catburger".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://mayaqq.dev".into(),
                                                                                                            img: "https://mayaqq.dev/assets/88x31/me.gif".into(),
                                                                                                            alt: "estrogen puppy".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://resolv.moe".into(),
                                                                                                            img: "https://resolv.moe/friends/resolv.png?v2".into(),
                                                                                                            alt: "Maybe".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://dynamicdispat.ch".into(),
                                                                                                            img: "https://dynamicdispat.ch".into(),
                                                                                                            alt: "dyn is cute and gay".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://awruff.dog".into(),
                                                                                                            img: "https://awruff.dog/assets/88x31.png".into(),
                                                                                                            alt: "shinyy ouppy :3".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                    },
                                                                )
                                                                .into(),
                                                        }),
                                                    ) {
                                                        x => {
                                                            (&&x)
                                                                .implements_render_or_display()
                                                                .render_to(x.0, &mut __maud_output)
                                                        }
                                                    }
                                                };
                                            },
                                        )
                                        .into(),
                                }),
                            ) {
                                x => {
                                    (&&x)
                                        .implements_render_or_display()
                                        .render_to(x.0, &mut __maud_output)
                                }
                            }
                        };
                        maud::PreEscaped(__maud_output)
                    }
                }
                let __cached_result = get_no_cache().await;
                let mut __cached_cached = GET.write().await;
                *__cached_cached = Some(
                    <Markup as ::std::clone::Clone>::clone(&__cached_result),
                );
                __cached_result
            }
            ///Primes the cached function [`get`].
            #[allow(dead_code)]
            pub async fn get_prime_cache() -> Markup {
                async fn get_no_cache() -> Markup {
                    let layout = {
                        crate::components::GridLayout::new(
                            crate::components::GridDefinition(
                                None,
                                &["links main main ", "links frens frens "],
                            ),
                            &[
                                crate::components::GridDefinition(
                                    Some(700),
                                    &["links main ", "links frens "],
                                ),
                                crate::components::GridDefinition(
                                    Some(530),
                                    &["main ", "links ", "frens "],
                                ),
                            ],
                        )
                    };
                    {
                        extern crate alloc;
                        extern crate maud;
                        let mut __maud_output = alloc::string::String::with_capacity(
                            2977usize,
                        );
                        {
                            use ::maud::macro_private::*;
                            match ChooseRenderOrDisplay(
                                &(Page {
                                    title: "invoking my kitty".into(),
                                    route: "/".into(),
                                    body: maud::macro_private::new_component_body(
                                            &|__maud_buffer: &mut String| {
                                                extern crate alloc;
                                                extern crate maud;
                                                let mut __maud_output: &mut alloc::string::String = {
                                                    __maud_buffer
                                                };
                                                {
                                                    use ::maud::macro_private::*;
                                                    match ChooseRenderOrDisplay(
                                                        &(Grid {
                                                            scope: ("seergysite::pages::index-29-25").into(),
                                                            layout: (layout).into(),
                                                            body: maud::macro_private::new_component_body(
                                                                    &|__maud_buffer: &mut String| {
                                                                        extern crate alloc;
                                                                        extern crate maud;
                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                            __maud_buffer
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "links".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                __maud_output
                                                                                                    .push_str(
                                                                                                        "<div class=\"box\"><ul class=\"icon-list fancy\">",
                                                                                                    );
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::T).into(),
                                                                                                            fg_force: "lavender".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://serenovv.tumblr.com\">Tumblr</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Photo).into(),
                                                                                                            fg_force: "peach".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://www.instagram.com/stupidcatcreacher/\">Instagram</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Music).into(),
                                                                                                            fg_force: "red".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://music.apple.com/profile/sery\">Apple Music</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::SourceCode).into(),
                                                                                                            fg_force: "sapphire".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://codeberg.org/sery\">Codebergs</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(IconLi {
                                                                                                            icon: (Icon::Cat).into(),
                                                                                                            fg_force: "text".into(),
                                                                                                            body: maud::macro_private::new_component_body(
                                                                                                                    &|__maud_buffer: &mut String| {
                                                                                                                        extern crate alloc;
                                                                                                                        extern crate maud;
                                                                                                                        let mut __maud_output: &mut alloc::string::String = {
                                                                                                                            __maud_buffer
                                                                                                                        };
                                                                                                                        __maud_output
                                                                                                                            .push_str(
                                                                                                                                "<a href=\"https://github.com/invokekitty\">Github</a>",
                                                                                                                            );
                                                                                                                    },
                                                                                                                )
                                                                                                                .into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                __maud_output.push_str("</ul></div>");
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "main".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                __maud_output
                                                                                                    .push_str(
                                                                                                        "<div class=\"box\"><h2>Welcome to sery's webbed site :3</h2><p>Haii im sery, dumbass cat that codes and enjoys music alot.\n                               You can find projects and random shit i say here :3</p><p>Im an enby cat that lives somewhere in Europe (wont tell u where :3). I like coding,\n                               mostly work on minecraft mods and other jvm-related stuff in kotlin, and pretty much anything\n                               else i ever feel like doing in rust (including <a href=\"https://codeberg.org/sery/webbed-site\">this site</a>!)</p><p>Uhmmm, i use linux btw :3. Currently using <a href=\"https://chimera-linux.org\">Chimera Linux</a>\n                               and <a href=\"https://cachyos.org\">CachyOS</a></p></div>",
                                                                                                    );
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                        {
                                                                            use ::maud::macro_private::*;
                                                                            match ChooseRenderOrDisplay(
                                                                                &(Area {
                                                                                    label: "frens".into(),
                                                                                    body: maud::macro_private::new_component_body(
                                                                                            &|__maud_buffer: &mut String| {
                                                                                                extern crate alloc;
                                                                                                extern crate maud;
                                                                                                let mut __maud_output: &mut alloc::string::String = {
                                                                                                    __maud_buffer
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "#".into(),
                                                                                                            tooltip: "Add me to your site :3".into(),
                                                                                                            img: "/banners/me.webp".into(),
                                                                                                            alt: "Me".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://kibbyc.at".into(),
                                                                                                            tooltip: "skeedge".into(),
                                                                                                            img: "/banners/riley.webp".into(),
                                                                                                            alt: "my cute catburger".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://mayaqq.dev".into(),
                                                                                                            img: "https://mayaqq.dev/assets/88x31/me.gif".into(),
                                                                                                            alt: "estrogen puppy".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://resolv.moe".into(),
                                                                                                            img: "https://resolv.moe/friends/resolv.png?v2".into(),
                                                                                                            alt: "Maybe".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://dynamicdispat.ch".into(),
                                                                                                            img: "https://dynamicdispat.ch".into(),
                                                                                                            alt: "dyn is cute and gay".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    use ::maud::macro_private::*;
                                                                                                    match ChooseRenderOrDisplay(
                                                                                                        &(Friendlink {
                                                                                                            href: "https://awruff.dog".into(),
                                                                                                            img: "https://awruff.dog/assets/88x31.png".into(),
                                                                                                            alt: "shinyy ouppy :3".into(),
                                                                                                        }),
                                                                                                    ) {
                                                                                                        x => {
                                                                                                            (&&x)
                                                                                                                .implements_render_or_display()
                                                                                                                .render_to(x.0, &mut __maud_output)
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                            },
                                                                                        )
                                                                                        .into(),
                                                                                }),
                                                                            ) {
                                                                                x => {
                                                                                    (&&x)
                                                                                        .implements_render_or_display()
                                                                                        .render_to(x.0, &mut __maud_output)
                                                                                }
                                                                            }
                                                                        };
                                                                    },
                                                                )
                                                                .into(),
                                                        }),
                                                    ) {
                                                        x => {
                                                            (&&x)
                                                                .implements_render_or_display()
                                                                .render_to(x.0, &mut __maud_output)
                                                        }
                                                    }
                                                };
                                            },
                                        )
                                        .into(),
                                }),
                            ) {
                                x => {
                                    (&&x)
                                        .implements_render_or_display()
                                        .render_to(x.0, &mut __maud_output)
                                }
                            }
                        };
                        maud::PreEscaped(__maud_output)
                    }
                }
                let __cached_result = get_no_cache().await;
                let mut __cached_cached = GET.write().await;
                *__cached_cached = Some(
                    <Markup as ::std::clone::Clone>::clone(&__cached_result),
                );
                __cached_result
            }
            let __resource = ::actix_web::Resource::new("/")
                .name("get")
                .guard(::actix_web::guard::Get())
                .to(get);
            ::actix_web::dev::HttpServiceFactory::register(__resource, __config);
        }
    }
    struct Friendlink<'a> {
        href: &'a str,
        img: &'a str,
        alt: &'a str,
        tooltip: Option<&'a str>,
    }
    impl Render for Friendlink<'_> {
        fn render_to(&self, buffer: &mut String) {
            {
                extern crate alloc;
                extern crate maud;
                let mut __maud_output: &mut alloc::string::String = { buffer };
                __maud_output.push_str("<a class=\"friendlink\" href=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&(self.href)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\"");
                if let Some(inner_value) = (self.tooltip) {
                    __maud_output.push_str(" data-tooltip=\"");
                    {
                        use ::maud::macro_private::*;
                        match ChooseRenderOrDisplay(&(inner_value)) {
                            x => {
                                (&&x)
                                    .implements_render_or_display()
                                    .render_to(x.0, &mut __maud_output)
                            }
                        }
                    };
                    __maud_output.push_str("\"");
                }
                __maud_output.push_str("><img src=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&(self.img)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\" alt=\"");
                {
                    use ::maud::macro_private::*;
                    match ChooseRenderOrDisplay(&(self.alt)) {
                        x => {
                            (&&x)
                                .implements_render_or_display()
                                .render_to(x.0, &mut __maud_output)
                        }
                    }
                };
                __maud_output.push_str("\"></a>");
            }
        }
    }
}
