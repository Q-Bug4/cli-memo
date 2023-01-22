use diesel::prelude::*;
use climemo::establish_connection;
use climemo::models::Memo;

fn main() {
    use climemo::schema::memos::dsl::*;

    let connection = &mut establish_connection();
    let results = memos
        .limit(5)
        .load::<Memo>(connection)
        .expect("Error loading memos");

    println!("Displaying {} memos", results.len());

    for memo in results {
        println!("{}", memo.name);
        println!("-----------\n");
        println!("{}", memo.content);
    }
}