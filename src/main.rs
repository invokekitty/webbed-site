#![feature(decl_macro, default_field_values)]
use std::{env, io};
use std::error::Error;
use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

mod styles;
mod pages;
mod layout;
mod components;
mod api;

#[derive(clap::Parser)]
#[command(version)]
struct Arguments {
    /// Ip address to bind my site to
    #[arg(short('i'), long, default_value_t="127.0.0.1".to_string())]
    bind_ip: String,
    /// Port to use
    #[arg(short, long, default_value_t=8080)]
    port: u16,
    /// My birthed date
    #[arg(long, default_value_t=Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap())]
    seergy_birth_date: DateTime<Utc>,
    /// Path to db file
    #[arg(long, default_value_t="seergysite.db".to_string())]
    db_path: String
}

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
    let pool = DbPool::builder()
        .build(ConnectionManager::new(args.db_path))?;

    HttpServer::new(move ||
        App::new()
            .app_data(pool.clone())
            .configure(pages::pages)
            .service(styles::style)
            .service(ResourceFiles::new("/", generate()))
        )
        .bind((args.bind_ip, args.port))?
        .run()
        .await
        .map_err(Box::from)
}
