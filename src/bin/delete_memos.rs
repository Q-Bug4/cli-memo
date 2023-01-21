use climemo::{delete_memo, establish_connection};

fn main() {
    let connection = &mut establish_connection();
    delete_memo(connection, 2);
}