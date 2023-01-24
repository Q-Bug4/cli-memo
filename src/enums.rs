use std::str::FromStr;
use strum_macros::EnumString;

#[derive(EnumString)]
#[derive(Debug)]
pub enum Language {
    #[strum(ascii_case_insensitive)]
    Python,

    #[strum(serialize = "js")]
    #[strum(ascii_case_insensitive)]
    JavaScript,

    #[strum(ascii_case_insensitive)]
    Bash,
}

#[derive(EnumString)]
#[derive(Debug)]
pub enum InputSourceEnum {
    #[strum(ascii_case_insensitive)]
    Text,

    #[strum(ascii_case_insensitive)]
    File,

    #[strum(ascii_case_insensitive)]
    FileSystem,

    #[strum(ascii_case_insensitive)]
    History,
}

#[derive(EnumString)]
#[derive(Debug)]
pub enum OutputResultEnum {
    #[strum(ascii_case_insensitive)]
    Text,

    #[strum(ascii_case_insensitive)]
    File,

    #[strum(ascii_case_insensitive)]
    FileSystem,
}

#[derive(EnumString)]
#[derive(Debug)]
pub enum Command {
    List,
    Create,
    Check,
    Modify,
    Remove,
    Run,
    Pick,
    Import,
}
