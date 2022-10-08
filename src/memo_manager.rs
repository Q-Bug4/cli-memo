use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

fn saveMemoToDisk(memo: Memo, path: PathBuf) {
    let filename = memo.name;
    path.push(filename);
    let mut file = File::create(path)?;
    file.write_all(memo.content);
    Ok(())
}

fn getMemoByName(name: String) -> Memo{

}

fn listMemos() -> &[Memo] {

}