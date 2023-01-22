pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Memo, NewMemo, UpdateMemo};
use crate::schema::memos;
use crate::schema::memos::{id};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_memo(conn: &mut SqliteConnection, memo: &Memo) {
    let new_memo = NewMemo {
        name: &memo.name,
        content: &memo.content,
        language: &memo.language,
        source_type: &memo.source_type,
        result_type: &memo.result_type,
    };

    let _ = diesel::insert_into(memos::table)
        .values(&new_memo)
        .execute(conn)
        .expect("Error saving new memo");
}

pub fn update_memo(conn: &mut SqliteConnection, memo_id: i32, memo: &UpdateMemo) {
    diesel::update(memos::table)
        .filter(id.is(memo_id))
        .set(memo)
        .execute(conn)
        .expect(&format!("Error updating memo with id: {}", memo_id));
}

pub fn delete_memo(conn: &mut SqliteConnection, memo_id: i32) {
    diesel::delete(memos::table)
        .filter(id.is(memo_id))
        .execute(conn)
        .expect(&format!("Error deleting memo with id: {}", memo_id));
}
