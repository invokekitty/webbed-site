#![feature(decl_macro, default_field_values, duration_constructors, duration_constants)]
#![feature(path_trailing_sep)]
#![feature(range_into_bounds)]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::error::Error;
use actix_web::web::{get, Data, QueryConfig};
use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;
use brotli::enc::BrotliEncoderParams;
use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey};
use ring::error::KeyRejected;
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
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
    /// Path to db file
    #[arg(long, default_value=".")]
    data_dir: PathBuf,
    /// My birthed date
    #[arg(long, default_value_t=Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap())]
    seergy_birth_date: DateTime<Utc>,
    /// Generate a jwt token and exit
    #[arg(long)]
    generate_token: Option<String>,
    #[arg(long)]
    reject_token: Option<String>
}

pub struct SiteData {
    pub database: SqlitePool,
    pub seergy_birth_date: DateTime<Utc>,
    pub decoding_key: DecodingKey
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mut args = Arguments::parse();
    args.data_dir.push_trailing_sep();
    let (encode_key, decode_key) = init_jwt_keypair(args.data_dir.with_file_name("skeegs.key"))?;
    let db = init_db(args.data_dir.with_file_name("skeegs.db")).await?;

    if let Some(sub) = args.generate_token {
        api::auth::generate_token(&db, sub, encode_key).await?;
    } else if let Some(sub) = args.reject_token {
        api::auth::reject_token(&db, sub).await?;
    }


    HttpServer::new(move || App::new()
            .app_data(Data::new(SiteData {
                database: db.clone(),
                seergy_birth_date: args.seergy_birth_date,
                decoding_key: decode_key.clone()
            }))
            .configure(pages::configure)
            .configure(api::blog::web::configure)
            .service(styles::style)
            .service(ResourceFiles::new("/assets", generate()))
            .default_service(get().to(pages::_404)))
        .bind((args.bind_ip, args.port))?
        .run()
        .await
        .map_err(Box::from)
}

fn init_jwt_keypair(key_file: impl AsRef<Path>) -> Result<(EncodingKey, DecodingKey), Box<dyn Error>> {
    let path = key_file.as_ref();
    if path.exists() {
        let pkcs8 = fs::read(path)?;
        let pair = Ed25519KeyPair::from_pkcs8(&pkcs8)?;

        return Ok((
            EncodingKey::from_ed_der(&pkcs8),
            DecodingKey::from_ed_der(&pair.public_key().as_ref())
        ));
    }

    let rng = SystemRandom::new();
    eprintln!("Key not present, generating");
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)?;
    let mut file = OpenOptions::new().create(true).write(true).open(path)?;
    #[cfg(target_family = "unix")] {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms)?;
    }
    file.write_all(pkcs8.as_ref())?;

    let pair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())?;

    Ok((
        EncodingKey::from_ed_der(pkcs8.as_ref()),
        DecodingKey::from_ed_der(&pair.public_key().as_ref())
    ))
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
            posted_on INTEGER NOT NULL,
            title TEXT NOT NULL,
            summary TEXT,
            content BLOB NOT NULL
        );
        CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY,
            post INTEGER NOT NULL,
            posted_on INTEGER NOT NULL,
            commenter TEXT NOT NULL,
            content BLOB NOT NULL,
            FOREIGN KEY(post) REFERENCES posts(id)
        );
        CREATE TABLE IF NOT EXISTS seergs (
            name TEXT PRIMARY KEY,
            created INTEGER NOT NULL,
            last_use INTEGER
        );
    "#)
    .execute(&pool)
    .await?;

    Ok(pool)
}
