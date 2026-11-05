use std::fmt::Write;

use maud::{ComponentBody, PreEscaped, Render, write_html};

pub const NBSP: PreEscaped<&str> = PreEscaped("&nbsp;");

pub macro scope() { concat!(module_path!(), "-", line!(), "-", column!()) }

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
        write_html! { buffer =>
            span.icon { (self.to_raw()) }
        }
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

pub struct IconLi<'a> {
    pub icon: Icon,
    pub tooltip: Option<&'a str> = None,
    pub fg: Option<&'a str> = None,
    pub bg: Option<&'a str> = None,
    pub fg_force: Option<&'a str> = None,
    pub fancy: bool = false,
    pub body: ComponentBody<'a>
}

impl Render for IconLi<'_> {
    fn render_to(&self, buffer: &mut String) {
        write_html! { buffer =>
            li .fancy[self.fancy] data-fg=[self.fg] data-fg-force=[self.fg_force] data-bg=[self.bg] data-tooltip=[self.tooltip] {
                (self.icon)
                span { (self.body) }
            }
        }
    }
}

pub macro grid($($($item:ident)+),+$(;$size:literal => [$($($sizeditem:ident)+),+])*) {{
    crate::components::GridLayout::new(
        crate::components::GridDefinition(None, &[$(concat!($(stringify!($item), " "),+)),+]),
        &[$(crate::components::GridDefinition(Some($size), &[$(concat!($(stringify!($sizeditem), " "),+)),+])),*]
    )
}}

pub struct Grid<'a> {
    pub scope: &'a str,
    pub layout: GridLayout<'a>,
    pub auto_rows: bool = false,
    pub aut_cols: bool = false,
    pub body: ComponentBody<'a>
}

impl Render for Grid<'_> {
    fn render_to(&self, buffer: &mut String) {
        let GridLayout { root, sized } = self.layout;
        write_html! { buffer =>
            style {
                (root.scoped(self.scope))
                @for definition in sized {
                    (definition.scoped(self.scope))
                }
            }
            div.grid .auto-rows[self.auto_rows] .auto-cols[self.aut_cols] data-scope=(self.scope) {
                (self.body)
            }
        }
    }
}

pub struct Area<'a> {
    pub id: Option<&'a str> = None,
    pub class: Option<&'a str> = None,
    pub label: &'a str,
    pub body: ComponentBody<'a>
}

impl Render for Area<'_> {
    fn render_to(&self, buffer: &mut String) {
        let area = "grid-area: ".to_owned() + self.label;
        write_html! { buffer =>
            div id=[self.id] class=[self.class] style=(area) {
                (self.body)
            }
        }
    }
}
#[derive(Clone, Copy)]
pub struct GridLayout<'a> {
    root: GridDefinition<'a>,
    sized: &'a [GridDefinition<'a>]
}

impl<'a> GridLayout<'a> {
    pub fn new(root: GridDefinition<'a>, sized: &'a [GridDefinition<'a>]) -> Self {
        GridLayout { root, sized }
    }
}

// WAAYY too complicated grid generator impl
#[derive(Clone, Copy)]
pub struct GridDefinition<'a>(pub Option<u16>, pub &'a [&'a str]);
impl<'a> GridDefinition<'a> {
    fn scoped(self, scope: &'a str) -> ScopedDefinition<'a> {
        ScopedDefinition(self, scope)
    }
}

pub struct ScopedDefinition<'a>(pub GridDefinition<'a>, pub &'a str);

impl Render for ScopedDefinition<'_> {
    fn render_to(&self, buffer: &mut String) {
        let ScopedDefinition(GridDefinition(size, rows), scope) = self;
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
//
//
// pub macro grid($($auto:ident;)?[$($row:literal),*];$($(@$px:literal[$($row_px:literal),*]);*;)?$($area:ident:{$($content:tt)*})*) {{
//     const SCOPE: &str = concat!(module_path!(), "-", line!(), "-", column!());
//     let _auto: bool = false $(|| stringify!($auto) == "auto")?;
//     let _auto_rows: bool = _auto $(|| stringify!($auto) == "autorows")?;
//     let _auto_cols: bool = _auto $(|| stringify!($auto) == "autocols")?;
//     let __rootdefinition = GridDefinition(None, &[$($row,)*], SCOPE);
//     let __sizeddefinitions: &[GridDefinition<'_>] = &[];
//     $(let __sizeddefinitions: &[GridDefinition<'_>] =
//         &[$(GridDefinition(Some($px),&[$($row_px,)*], SCOPE)),*];)?
//
//     maud::html! {
//         style {
//             (__rootdefinition)
//             @for _sd in __sizeddefinitions {
//                 (_sd)
//             }
//         }
//         .grid.auto-columns[_auto_cols].auto-rows[_auto_rows] data-scope=(SCOPE) {
//             $(div data-scope=(SCOPE) style=(concat!("grid-area:", stringify!($area), ";")) {
//                 $($content)*
//             })*
//         }
//     }
// }}
