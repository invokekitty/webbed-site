use chrono::NaiveDate;
use diesel::{table, Queryable, QueryDsl, RunQueryDsl};

table! {
    posts (author_id, post_id) {
        author_id -> BigInt,
        post_id -> BigInt,
        date -> Date,
        title -> Text,
        desc -> Nullable<Text>,
        content -> Text
    }
}

#[derive(Queryable)]
struct Post {
    author_id: i64,
    post_id: i64,
    date: NaiveDate,
    title: String,
    desc: Option<String>,
    content: String
}

fn meow() {

    let x = posts::table.grou

}