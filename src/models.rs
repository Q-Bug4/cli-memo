use diesel::prelude::*;
use crate::schema::memos;

#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub language: String,
    pub source_type: String,
    pub result_type: String,
}

#[derive(Insertable)]
#[diesel(table_name = memos)]
pub struct NewMemo<'a> {
    pub name: &'a str,
    pub content: &'a str,
    pub language: &'a str,
    pub source_type: &'a str,
    pub result_type: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = memos)]
pub struct UpdateMemo<'a> {
    pub name: Option<&'a str>,
    pub content: Option<&'a str>,
    pub language: Option<&'a str>,
    pub source_type: Option<&'a str>,
    pub result_type: Option<&'a str>,
}
