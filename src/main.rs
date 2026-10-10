#![feature(decl_macro, macro_metavar_expr)]
use std::io;

use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use clap::Parser;

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
    port: u16
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = Arguments::parse();

    HttpServer::new(|| {
        App::new()
            .service(pages::index)
            .service(styles::style)
            .service(ResourceFiles::new("/assets", generate()))
    })
    .bind((args.bind_ip, args.port))?
    .run()
    .await
}
