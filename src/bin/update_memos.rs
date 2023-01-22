use diesel::prelude::*;
use climemo::{establish_connection, update_memo};
use climemo::models::{Memo, UpdateMemo};
use climemo::schema::memos::dsl::*;

fn main() {
    let memo = UpdateMemo {
        name: None,
        content: Some("updated memo 23:54"),
        language: Some("Markdown"),
        source_type: None,
        result_type: None,
    };
    let connection = &mut establish_connection();
    update_memo(connection, 1, &memo);
}