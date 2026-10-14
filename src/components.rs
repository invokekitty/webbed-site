use std::fmt::Write;

use maud::{Markup, PreEscaped, Render, html};

pub const NBSP: PreEscaped<&str> = PreEscaped("&nbsp;");

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Icon {
    Goblin = '' as u32,
    Cpu = '' as u32,
    Ram = '' as u32,
    Menu = '' as u32,
    Music = '' as u32,
    Photo = '' as u32,
    Cat = '' as u32,
    Arch = '' as u32,
    Note = '' as u32,
    Coffee = '' as u32,
    Heart = '' as u32,
    HeartSmall = '' as u32,
    SourceCode = '' as u32,
    Updoot = '' as u32,
    Downdoot = '' as u32,
    Social = '' as u32,
    Group = '' as u32,
    Quoip = '' as u32,
    DoubleCheck = '' as u32,
    City = '' as u32,
    IUseArchBtw = '' as u32,
    Cog = '' as u32,
    Drop = '' as u32,
    ShoppingCartIThink = '' as u32,
    Safari = '' as u32,
    Warning = '' as u32,
    Server = '' as u32,
    Download = '' as u32,
    Upload = '' as u32,
    CD = '' as u32,
    Floppy = '' as u32,
    T = 't' as u32
}

impl Icon {
    pub fn to_raw(&self) -> impl Render {
        RawIcon(*self)
    }
}

impl Render for Icon {
    // fn render(&self) -> maud::Markup {
    //     let chr = char::from_u32(*self as u32).expect("I fuckied upied");
    //     html! { span.icon { (chr) } }
    // } is what i would do, if i wasnt an insane performance freak
    fn render_to(&self, buffer: &mut String) {
        buffer.push_str("<span class=\"icon\">");
        self.to_raw().render_to(buffer);
        buffer.push_str("</span>");
    }
}

struct RawIcon(Icon);
impl Render for RawIcon {
    fn render_to(&self, buffer: &mut String) {
        let chr = unsafe { char::from_u32_unchecked((self.0) as u32) };
        if let Icon::T = self.0 {
            buffer.push_str("<b style=\"font-size: 1.2rem\">&nbsp;");
        }
        buffer.push(chr);
        if let Icon::T = self.0 {
            buffer.push_str("</b>");
        }
    }
}

pub macro iconli($icon:ident $(@$tooltip:literal)? $([$($attrs:tt)+])? {$($content:tt)*}) {{
    maud::html! {
        li $($($attrs)+)? data-icon-list {
            (Icon::$icon)
            span { $($content)* }
        }
    }
}}

pub macro iconlist($([$($props:tt)+])?$($icon:ident $(@[$($iconprops:tt)+])? $([$($liprops:tt)+])? {$($content:tt)*})*) {{
    maud::html! {
        ul.icon-list $($($props)+)? {$(
            li $($($liprops)+)? {
                span.icon $($($iconprops)+)? { (Icon::$icon.to_raw()) }
                span { $($content)* }
            }
        )*}
    }
}}

pub fn friendlink(href: &str, tooltip: Option<&str>, img: &str, alt: &str) -> Markup {
    html! {
        a.friendlink href=(href) data-tooltip=[tooltip] {
            img src=(img) alt=(alt);
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

pub fn project(id: &str, links: &[(Platform, &str)], deprecated: bool, content: Markup) -> Markup {
    html! {
        div #(id) .project {
            @if deprecated {
                span.deprecated-marker { (Icon::Warning)(NBSP)"Deprecated" }
            }
            (content)
            span.fancy {
                @for (index, (platform, link)) in links.iter().enumerate() {
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

// WAAYY too complicated grid generator impl
pub struct GridDefinition<'a>(pub Option<u32>, pub &'a [&'a str], pub &'static str);

impl Render for GridDefinition<'_> {
    fn render_to(&self, buffer: &mut String) {
        let GridDefinition(size, rows, scope) = self;
        let mut depth: u8 = 0;
        if let Some(size) = size {
            depth += 1;
            let _ = write!(buffer, "@media(max-width: {size}px){{");
        }
        depth += 1;
        let _ = write!(buffer, ".grid[data-scope=\"{scope}\"]{{");
        buffer.push_str("grid-template-areas:");
        for row in rows.iter() {
            buffer.push(' ');
            let _ = write!(buffer, "\"{row}\"");
        }
        buffer.push(';');

        for _ in 0..depth {
            buffer.push('}');
        }
    }
}

pub macro grid($($auto:ident;)?[$($row:literal),*];$($(@$px:literal[$($row_px:literal),*]);*;)?$($area:ident:{$($content:tt)*})*) {{
    const SCOPE: &str = concat!(module_path!(), "-", line!(), "-", column!());
    let _auto: bool = false $(|| stringify!($auto) == "auto")?;
    let _auto_rows: bool = _auto $(|| stringify!($auto) == "autorows")?;
    let _auto_cols: bool = _auto $(|| stringify!($auto) == "autocols")?;
    let __rootdefinition = GridDefinition(None, &[$($row,)*], SCOPE);
    let __sizeddefinitions: &[GridDefinition<'_>] = &[];
    $(let __sizeddefinitions: &[GridDefinition<'_>] =
        &[$(GridDefinition(Some($px),&[$($row_px,)*], SCOPE)),*];)?

    maud::html! {
        style {
            (__rootdefinition)
            @for _sd in __sizeddefinitions {
                (_sd)
            }
        }
        .grid.auto-columns[_auto_cols].auto-rows[_auto_rows] data-scope=(SCOPE) {
            $(div data-scope=(SCOPE) style=(concat!("grid-area:", stringify!($area), ";")) {
                $($content)*
            })*
        }
    }
}}
