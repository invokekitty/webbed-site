use std::{error::Error, hash::Hash, io};
use std::ops::{Bound, IntoBounds, RangeFrom, RangeTo};
use std::range::Range;
use brotli::{BrotliCompress, BrotliDecompress, enc::{BrotliEncoderParams, backward_references::BrotliEncoderMode}};
use cached::{concurrent_cached, once};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{encode::IsNull, prelude::{FromRow, Type}, query, query_as, sqlite::{SqliteArgumentsBuffer, SqliteQueryResult, SqliteValueRef}, Decode, Encode, Row, Sqlite, SqlitePool};
use sqlx::query::Query;

#[concurrent_cached(
    key = "u64",
    convert = r#"{id}"#,
    max_size = 1024
)]
pub async fn fetch_post(db_pool: &SqlitePool, id: u64) -> Result<Option<PostData>, sqlx::Error> {
    query_as("SELECT id, posted_on, title, summary, content FROM posts WHERE id = $1")
        .bind(id as i64)
        .fetch_optional(db_pool)
        .await
}

#[once]
pub async fn get_all_posts(db_pool: &SqlitePool) -> Result<Vec<PostData>, sqlx::Error> {
    query_as("SELECT id, posted_on, title, summary, content FROM posts")
        .fetch_all(db_pool)
        .await
}

pub async fn list_post_ids(db: &SqlitePool, after: Option<DateTime<Utc>>, before: Option<DateTime<Utc>>) -> Result<Vec<u64>, sqlx::Error> {
    let query: Query<Sqlite, _> = match (before, after) {
        (Some(after), Some(before)) =>
            query("SELECT id FROM posts WHERE posted_on > $1 AND posted_on < $2")
                .bind(after)
                .bind(before),
        (Some(after), None) =>
            query("SELECT id FROM posts WHERE posted_on > $1")
                .bind(after),
        (None, Some(before)) =>
            query("SELECT id FROM posts WHERE posted_on < $1")
                .bind(before),
        (None, None) =>
            query("SELECT id FROM posts")
    };

    let r: Vec<u64> = query.fetch_all(db)
        .await?
        .into_iter()
        .map(|eeg| eeg.get::<i64, _>(0) as u64)
        .collect();

    Ok(r)
}

pub mod web {
    use std::error::Error;
    use actix_web::{web, get, HttpRequest, post, Responder};
    use actix_web::web::{Data, Json, ServiceConfig};
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use crate::api::blog::{fetch_post, PostData, list_post_ids};
    use crate::SiteData;

    pub fn configure(config: &mut ServiceConfig) {
        config.service(get_post)
            .service(create_post)
            .service(list_posts);
    }


    #[derive(Copy, Clone, Deserialize)]
    struct PostsQuery {
        after: Option<DateTime<Utc>>,
        before: Option<DateTime<Utc>>
    }

    #[get("/api/posts/list")]
    async fn list_posts(
        query: web::Query<PostsQuery>,
        site_data: Data<SiteData>
    ) -> Result<impl Responder, Box<dyn Error>> {
        Ok(Json(list_post_ids(&site_data.database, query.after, query.before).await?))
    }

    #[post("/api/posts/create")]
    async fn create_post(
        req: HttpRequest,
        Json(mut post): Json<PostData>,
        site_data: Data<SiteData>
    ) -> Result<impl Responder, Box<dyn Error>> {
        post.post(&site_data.database).await?;
        Ok(Json(post))
    }

    #[get("/api/posts/post-{id}")]
    async fn get_post(id: web::Path<u64>, site_data: Data<SiteData>) -> Result<impl Responder, Box<dyn Error>> {
        Ok(Json(fetch_post(&site_data.database, *id).await?))
    }

}


#[derive(Clone, FromRow, Type, Serialize, Deserialize)]
pub struct PostData {
    id: Option<u64>,
    pub posted_on: DateTime<Utc>,
    pub title: String,
    pub summary: Option<String>,
    pub content: CompressableText
}

impl PostData {
    pub fn new(title: impl Into<String>, desc: Option<String>, content: impl Into<String>) -> Self {
        PostData { 
            id: None, 
            posted_on: Utc::now(),
            title: title.into(),
            summary: desc,
            content: CompressableText(content.into())
        }
    }

    pub async fn post(&mut self, db_pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
        if let Some(_) = &self.id {
            return Err("The post already has a database id, use modify methods instead".into());
        }
        
        let result = query("INSERT INTO posts (posted_on, title, summary, content) VALUES ($1, $2, $3, $4)")
            .bind(&self.posted_on)
            .bind(&self.title)
            .bind(&self.summary)
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

    pub async fn update_title_and_summary(&self, db: &SqlitePool) -> Result<SqliteQueryResult, Box<dyn Error>> {
        if let Some(id) = self.id {
            let result = query("UPDATE posts SET title = $1, desc = $2 WHERE id = $3")
                .bind(&self.title)
                .bind(&self.summary)
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
pub struct CompressableText(pub String);

impl CompressableText {
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
        Ok(CompressableText(String::from_utf8(buf)?))
    }

    fn compress(&self) -> Result<Vec<u8>, io::Error> {
        Self::compress_inner(&self.0)
    }
}

impl Into<String> for CompressableText {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for CompressableText {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> Type<Sqlite> for CompressableText {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <Vec<u8> as Type<Sqlite>>::type_info()
    }
}

impl<'a> Decode<'a, Sqlite> for CompressableText {
    fn decode(value: SqliteValueRef<'a>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <&[u8] as Decode<'a, Sqlite>>::decode(value)?;
        Ok(Self::decompress(bytes)?)
    }
}

impl<'a> Encode<'a, Sqlite> for CompressableText {
    fn encode_by_ref(
        &self,
        buf: &mut SqliteArgumentsBuffer,
    ) -> Result<IsNull, sqlx::error::BoxDynError>
    {
        <Vec<u8> as Encode<'a, Sqlite>>::encode(self.compress()?, buf)
    }
}
