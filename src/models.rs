use diesel::prelude::*;

#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub language: String,
    pub source_type: String,
    pub result_type: String,
}
