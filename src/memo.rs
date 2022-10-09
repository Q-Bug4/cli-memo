use r_lombok_macros::{Getter, Setter};
use crate::constant::{InputSourceEnum, Language, OutputResultEnum};


#[derive(Getter, Setter)]
pub struct Memo {
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) language: Language,
    // Should be enum
    pub(crate) source_type: InputSourceEnum,
    // Should be enum
    pub(crate) result_type: OutputResultEnum, // Should be enum
}


