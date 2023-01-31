pub mod models;
pub mod schema;
pub mod constant;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, fs};
use std::path::Path;
use std::process::Command;
use crate::constant::DEFAULT_SCRIPT_FOLDER;
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

pub fn exec_python_script(script_path: &str) -> String {
    let output = Command::new("python")
        .arg(script_path)
        .output()
        .expect("failed to execute process");
    match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Fail to read from utf-8, {}", e),
    }
}

pub fn init_script_files() {
    let home_dir = dirs::home_dir().expect("Fail to get home dir");
    let script_path = home_dir.as_path().join(constant::DEFAULT_SCRIPT_FOLDER);
    fs::create_dir_all(script_path.to_str().unwrap()).expect("Fail to init cli-memo dir");
}
