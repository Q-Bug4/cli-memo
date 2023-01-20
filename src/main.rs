#![allow(unused)]

mod memo;
mod constant;

use structopt::StructOpt;
use crate::memo::Memo;
use crate::constant::{InputSourceEnum, Language, OutputResultEnum};

#[derive(StructOpt)]
struct Cli {
    cmd: String,
}

fn main() {
    let memo = Memo {
        name: "TestMemo".to_string(),
        content: "test1".to_string(),
        language: Language::JavaScript,
        source_type: InputSourceEnum::Text,
        result_type: OutputResultEnum::Text,
    };
    memo.insert();
}
