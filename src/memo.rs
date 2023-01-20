use crate::constant::{InputSourceEnum, Language, OutputResultEnum};


#[derive(Debug)]
pub struct Memo {
    pub(crate) name: String,
    pub(crate) content: String,
    pub(crate) language: Language, // Should be enum
    pub(crate) source_type: InputSourceEnum, // Should be enum
    pub(crate) result_type: OutputResultEnum, // Should be enum
}

impl Memo {
    pub fn insert(self: &Self) {
        println!("{self:?}")
    }

    pub fn update(self: &Self) {

    }
}
