use serde::Deserialize;
use serde_json;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
