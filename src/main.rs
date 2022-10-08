#![allow(unused)]
use structopt::StructOpt;
#[path ="./test/memo_manager.rs"]
mod test_memo_manager;

#[derive(StructOpt)]
struct Cli {
    cmd: String,
}

fn main() {
    let args = Cli::from_args();
    let cmd = args.cmd;
}
