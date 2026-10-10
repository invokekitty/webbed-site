use std::{collections::HashMap, fmt::Write};

use maud::{Markup, Render, html};

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
    Floppy = '' as u32
}

impl Render for Icon {
    fn render(&self) -> maud::Markup {
        let chr = char::from_u32(*self as u32).expect("I fuckied upied");
        html! { span.icon { (chr) } }
    }
}
