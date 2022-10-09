use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

use crate::memo::Memo;

pub fn saveMemoToDisk(memo: Memo, mut path: PathBuf) -> std::io::Result<()> {
    let filename = memo.get_name();
    path.push(filename);
    let mut file = File::create(path)?;
    file.write_all(memo.get_content().as_bytes());
    Ok(())
}

// fn getMemoByName(name: String) -> Memo{
//
// }
//
// fn listMemos() -> &[Memo] {
//
// }