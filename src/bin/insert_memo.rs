use climemo::{create_memo, establish_connection};
use climemo::models::Memo;

fn main() {
    let memo = Memo{
        id: 0,
        name: "name".to_string(),
        content: "print(1)".to_string(),
        language: "python".to_string(),
        source_type: "TEXT".to_string(),
        result_type: "TEXT".to_string(),
    };
    let mut conn = establish_connection();
    create_memo(&mut conn, &memo);
}