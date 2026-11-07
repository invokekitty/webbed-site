#![feature(decl_macro, default_field_values)]
use std::path::Path;
use std::sync::Arc;
use std::{env, io};
use std::error::Error;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use brotli::enc::BrotliEncoderParams;
use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use sqlx::sqlite::{SqliteConnectOptions, SqliteError, SqlitePoolOptions};
use sqlx::{ConnectOptions, Sqlite, SqliteConnection, SqlitePool, migrate, query, raw_sql, sqlite};

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
    #[arg(short('i'), long, default_value_t="localhost".to_string())]
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

struct SiteData {
    database: SqlitePool,
    seergy_birth_date: DateTime<Utc>
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args = Arguments::parse();
    let dbpool = init_db(args.db_path).await?;

    HttpServer::new(move || App::new()
            .app_data(Data::new(SiteData { 
                database: dbpool.clone(),
                seergy_birth_date: args.seergy_birth_date
            }))
            .configure(pages::configure)
            .service(styles::style)
            .service(ResourceFiles::new("/", generate())))
        .bind((args.bind_ip, args.port))?
        .run()
        .await
        .map_err(Box::from)
}


async fn init_db(db_path: impl AsRef<Path>) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true)
                .foreign_keys(true))
        .await?;    

    raw_sql(r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY,
            date INTEGER NOT NULL,
            title TEXT NOT NULL,
            summary TEXT,
            content BLOB NOT NULL
        );
        CREATE TABLE IF NOT EXISTS comments (
            
        );
    "#)
    .execute(&pool)
    .await?;

    Ok(pool)
}
