use crate::constant::{InputSourceEnum, Language, OutputResultEnum};


struct Memo {
    name: String,
    content: String,
    language: Language, // Should be enum
    source_type: InputSourceEnum, // Should be enum
    result_type: OutputResultEnum, // Should be enum
}


