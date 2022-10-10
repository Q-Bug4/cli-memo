#[cfg(test)]
#[path ="../constant.rs"]
use crate::constant::{InputSourceEnum, Language, OutputResultEnum};
mod memo_manager_test {
    use std::path::PathBuf;
    use crate::constant::InputSourceEnum;
    use crate::constant::Language::Python;
    use crate::memo::Memo;
    use crate::memo_manager;
    use super::*;

    #[test]
    fn test_save_memo() {
        let memo = Memo {
            name: "test_memo".parse().unwrap(),
            language: Python,
            content: "print('hi')".parse().unwrap(),
            source_type: InputSourceEnum::Text,
            result_type: OutputResultEnum::Text
        };
        let mut path: PathBuf = dirs::home_dir().expect("cant get home dir");
        path.push("tmp");
        memo_manager::saveMemoToDisk(memo, path);
    }
}