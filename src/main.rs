#![allow(unused)]

mod constant;

use structopt::StructOpt;
use crate::memo::Memo;
use crate::constant::{InputSourceEnum, Language, OutputResultEnum};

#[derive(StructOpt)]
struct Cli {
    cmd: String,
}

fn main() {
}
