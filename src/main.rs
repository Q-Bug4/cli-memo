#![allow(unused)]
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    cmd: String,
}

fn main() {
    let args = Cli::from_args();
    let cmd = args.cmd;
}
