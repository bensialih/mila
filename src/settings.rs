use std::{io::Read, path::PathBuf};

use crate::files::FileObj;
use serde::Deserialize;
use serde_json;
use std::fs::File;

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileSize {
    Mb(u64),
    Kb(u64),
    Bytes(u64),
}

impl From<FileSize> for u64 {
    fn from(value: FileSize) -> Self {
        match value {
            FileSize::Mb(number) => number * 1_000_000,
            FileSize::Kb(number) => number * 1_000,
            FileSize::Bytes(number) => number,
        }
    }
}

impl From<FileObj> for FileSize {
    fn from(value: FileObj) -> Self {
        let path = PathBuf::from(value);
        let file = File::open(path).expect("expected valid file");
        let size: FileSize = serde_json::from_reader(file).unwrap();
        return size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::{FolderOperator, FolderTrait};
    use std::io::Write;

    #[test]
    fn test_string_parsing() {
        let data = r#"{"kb": 100}"#;
        let size: FileSize = serde_json::from_str(data).unwrap();
        assert_eq!(FileSize::Kb(100), size);
    }

    #[test]
    fn test_fail_parse() {
        let data = r#"{"unknown": 100}"#;
        let result = serde_json::from_str::<FileSize>(data);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_file_parsed() {
        let data = r#"{"kb": 100}"#;
        let builder = FolderOperator::new().with_directory("./serde_test");
        let path = PathBuf::from("./serde_test/content.json");
        File::create(&path).unwrap().write(data.as_bytes());
        let size: FileSize = FileSize::from(FileObj::from(path));

        assert_eq!(FileSize::Kb(100), size);
        builder.delete();
    }
}
