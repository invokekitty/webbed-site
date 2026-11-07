use std::{borrow::Cow, error::Error, hash::Hash, io};

use brotli::{BrotliCompress, BrotliDecompress, enc::{BrotliEncoderParams, backward_references::BrotliEncoderMode}};
use cached::{concurrent_cached, once};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode, Sqlite, SqlitePool, encode::IsNull, prelude::{FromRow, Type}, query, query_as, sqlite::{SqliteArgumentsBuffer, SqliteQueryResult, SqliteTypeInfo, SqliteValueRef}};

#[concurrent_cached(
    key = "u64",
    convert = r#"{id}"#,
    max_size = 1024
)]
pub async fn fetch_post(db_pool: &SqlitePool, id: u64) -> Result<Option<PostData>, sqlx::Error> {
    query_as("SELECT (id, date, title, desc, content) FROM posts WHERE id = $1")
        .bind(id as i64)
        .fetch_optional(db_pool)
        .await
}

#[once]
pub async fn get_all_posts(db_pool: &SqlitePool) -> Result<Vec<PostData>, sqlx::Error> {
    query_as("SELECT (id, date, title, desc, content) FROM posts")
        .fetch_all(db_pool)
        .await
}


#[derive(Clone, FromRow, Type, Serialize, Deserialize)]
pub struct PostData {
    id: Option<u64>,
    pub date: DateTime<Utc>,
    pub title: String,
    pub desc: Option<String>,
    pub content: PostBody
}

impl PostData {
    pub fn new(title: impl Into<String>, desc: Option<String>, content: impl Into<String>) -> Self {
        PostData { 
            id: None, 
            date: Utc::now(), 
            title: title.into(), 
            desc, 
            content: PostBody(content.into()) 
        }
    }

    pub async fn post(&mut self, db_pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
        if let Some(_) = &self.id {
            return Err("The post already has a database id, use modify methods instead".into());
        }
        
        let result = query("INSERT INTO posts (date, title, desc, content) VALUES ($1, $2, $3, $4)")
            .bind(&self.date)
            .bind(&self.title)
            .bind(&self.desc)
            .bind(&self.content)
            .execute(db_pool)
            .await?;

        let id = result.last_insert_rowid() as u64;
        self.id = Some(id);
        self.invalidate_caches().await;
        Ok(())
    }

    pub async fn update_content(&self, db: &SqlitePool) -> Result<SqliteQueryResult, Box<dyn Error>> {
        if let Some(id) = self.id {
            let result = query("UPDATE posts SET content = $1 WHERE id = $2")
                .bind(&self.content)
                .bind(id as i64)
                .execute(db)
                .await?;
            self.invalidate_caches().await;
            Ok(result)
        } else {
            Err("Not a post in the database".into())
        }
    }

    pub async fn update_title_and_desc(&self, db: &SqlitePool) -> Result<SqliteQueryResult, Box<dyn Error>> {
        if let Some(id) = self.id {
            let result = query("UPDATE posts SET title = $1, desc = $2 WHERE id = $3")
                .bind(&self.title)
                .bind(&self.desc)
                .bind(id as i64)
                .execute(db)
                .await?;
            self.invalidate_caches().await;
            Ok(result)
        } else {
            Err("Not a post in the database".into())
        }
    }

    async fn invalidate_caches(&self) {
        let id = self.id.expect("Should only be called on posts in the database");
        if let Some(posts_cache) = FETCH_POST.get() {
            posts_cache.set(id, Some(self.clone()));
        }
    
        *GET_ALL_POSTS.write().await = None;
    }
}

impl PartialEq for PostData {
    fn eq(&self, other: &Self) -> bool {
        if let Some(id) = self.id && let Some(other_id) = other.id {
            id == other_id
        } else {
            false
        }
    }
}

impl Hash for PostData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(id) = self.id {
            state.write_u64(id);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PostBody(pub String);

impl PostBody {
    fn compress_inner(string: &str) -> Result<Vec<u8>, io::Error> {
        let mut compressed = Vec::<u8>::with_capacity(string.len());
        let mut params = BrotliEncoderParams::default();
        params.mode = BrotliEncoderMode::BROTLI_MODE_TEXT;
        params.quality = 8;
        BrotliCompress(&mut string.as_bytes(), &mut compressed, &params)?;
        Ok(compressed)
    }

    fn decompress<'a>(mut source: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut buf = Vec::<u8>::with_capacity(source.len() * 2);
        BrotliDecompress(&mut source, &mut buf)?;
        Ok(PostBody(String::from_utf8(buf)?))
    }

    fn compress(&self) -> Result<Vec<u8>, io::Error> {
        Self::compress_inner(&self.0)
    }
}

impl Into<String> for PostBody {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for PostBody {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> Type<Sqlite> for PostBody {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
}

impl<'a> Decode<'a, Sqlite> for PostBody {
    fn decode(value: SqliteValueRef<'a>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <&[u8] as Decode<'a, Sqlite>>::decode(value)?;
        Ok(Self::decompress(bytes)?)
    }
}

impl<'a> Encode<'a, Sqlite> for PostBody {
    fn encode_by_ref(
        &self,
        buf: &mut SqliteArgumentsBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError>
    {
        <Vec<u8> as Encode<'a, Sqlite>>::encode(self.compress()?, buf)
    }
}
