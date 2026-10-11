#![feature(decl_macro, macro_metavar_expr)]
use std::io;

use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use chrono::{DateTime, Local, TimeZone, Utc};
use clap::Parser;

use crate::pages::SEERGYBIRTH;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod styles;
mod pages;
mod layout;
mod components;

#[derive(clap::Parser)]
#[command(version)]
struct Arguments {
    #[arg(short('i'), long, default_value_t="127.0.0.1".to_string())]
    bind_ip: String,
    #[arg(short, long, default_value_t=8080)]
    port: u16,
    #[arg(short, long, default_value_t=Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap())]
    seergy_birth_date: DateTime<Utc>
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = Arguments::parse();
    let _ = SEERGYBIRTH.set(args.seergy_birth_date);

    HttpServer::new(|| {
        App::new()
            .service(pages::index)
            .service(pages::me)
            .service(styles::style)
            .service(ResourceFiles::new("/", generate()))
    })
    .bind((args.bind_ip, args.port))?
    .run()
    .await
}
