/// Modifies the human hash string according to the TextClass
pub fn modify_human_hash_for_class(text_class: &TextClass, hash: &str) -> String {
    match text_class {
        TextClass::Email => format!("{}@example.com", hash.replace(' ', ".")),
        TextClass::Url => format!("https://{}/", hash.replace(' ', "-")),
        TextClass::Numeric => hash.chars().filter(|c| c.is_ascii_digit()).collect(),
        TextClass::Alpha => hash.chars().filter(|c| c.is_ascii_alphabetic()).collect(),
        TextClass::Alphanumeric => hash.chars().filter(|c| c.is_ascii_alphanumeric()).collect(),
        TextClass::Other => hash.to_string(),
    }
}
use human_hash::HumanHasher;
use regex::Regex;
use lazy_static::lazy_static;
use serde_json::{Deserializer, Value};
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::io::{self, BufRead, Write};
use uuid::Uuid;

/// Enum to classify the type of text for obfuscation purposes
#[derive(Debug, PartialEq, Eq)]
pub enum TextClass {
    Email,
    Url,
    Numeric,
    Alpha,
    Alphanumeric,
    Other,
}

/// Classifies a string into a TextClass
pub fn classify_text(s: &str) -> TextClass {
    lazy_static! {
        static ref EMAIL_RE: Regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        static ref URL_RE: Regex = Regex::new(r"^(https?://)?[\w\.-]+\.[a-zA-Z]{2,}(/\S*)?$" ).unwrap();
    }
    if EMAIL_RE.is_match(s) {
        TextClass::Email
    } else if URL_RE.is_match(s) {
        TextClass::Url
    } else if s.chars().all(|c| c.is_ascii_digit()) {
        TextClass::Numeric
    } else if s.chars().all(|c| c.is_ascii_alphabetic()) {
        TextClass::Alpha
    } else if s.chars().all(|c| c.is_ascii_alphanumeric()) {
        TextClass::Alphanumeric
    } else {
        TextClass::Other
    }
}

/// Checks if a string is in ISO8601 datetime format.
pub fn is_datetime(s: &str) -> bool {
    // Basic ISO8601 regex: 2023-07-16T19:20:30Z or 2023-07-16T19:20:30+01:00
    // This is a simplified version and may be adjusted for stricter matching
    lazy_static! {
        static ref DATETIME_RE: Regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})$" ).unwrap();
    }
    DATETIME_RE.is_match(s)
}

fn obfuscate_value(val: &mut Value) {
    match val {
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                obfuscate_value(v);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                obfuscate_value(v);
            }
        }
        Value::String(s) => {
            if is_datetime(s) {
                // Leave ISO8601 datetime strings intact
                return;
            }
            let text_class = classify_text(s);
            let mut hasher = DefaultHasher::new();
            let word_count = s.split_whitespace().count();
            hasher.write(s.as_bytes());
            let hash = hasher.finish();
            let human_hash = HumanHasher::new(human_hash::DEFAULT_WORDLIST)
                .humanize(&Uuid::from_u64_pair(hash, hash.wrapping_add(1)), word_count)
                .replace("-", " ");
            let obfuscated = modify_human_hash_for_class(&text_class, &human_hash);
            *s = obfuscated;
        }
        _ => {}
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let handle = stdout.lock();
    let reader = stdin.lock();
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();
    let mut writer = io::BufWriter::new(handle);

    for value in stream {
        match value {
            Ok(mut val) => {
                obfuscate_value(&mut val); // Clone to avoid borrowing issues
                serde_json::to_writer_pretty(&mut writer, &val)
                    .expect("Failed to write JSON with pretty format");
                writer.write_all(b"\n").expect("Failed to write newline");
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{is_datetime, classify_text, TextClass, modify_human_hash_for_class};
    #[test]
    fn test_modify_human_hash_for_class_email() {
        let hash = "red apple";
        let result = modify_human_hash_for_class(&TextClass::Email, hash);
        assert_eq!(result, "red.apple@example.com");
    }

    #[test]
    fn test_modify_human_hash_for_class_url() {
        let hash = "blue sky";
        let result = modify_human_hash_for_class(&TextClass::Url, hash);
        assert_eq!(result, "https://blue-sky/");
    }

    #[test]
    fn test_modify_human_hash_for_class_numeric() {
        let hash = "foo bar";
        let result = modify_human_hash_for_class(&TextClass::Numeric, hash);
        // No digits in hash, so result should be empty
        assert_eq!(result, "");
    }

    #[test]
    fn test_modify_human_hash_for_class_alpha() {
        let hash = "foo bar";
        let result = modify_human_hash_for_class(&TextClass::Alpha, hash);
        assert_eq!(result, "foobar");
    }

    #[test]
    fn test_modify_human_hash_for_class_alphanumeric() {
        let hash = "foo!bar";
        let result = modify_human_hash_for_class(&TextClass::Alphanumeric, hash);
        assert_eq!(result, "foobar");
    }

    #[test]
    fn test_modify_human_hash_for_class_other() {
        let hash = "foo bar!@#";
        let result = modify_human_hash_for_class(&TextClass::Other, hash);
        assert_eq!(result, "foo bar!@#");
    }

    #[test]
    fn test_is_datetime() {
        assert!(is_datetime("2023-07-16T19:20:30Z"));
        assert!(is_datetime("2023-07-16T19:20:30+01:00"));
        assert!(is_datetime("2023-07-16T19:20:30-05:00"));
        assert!(is_datetime("2023-07-16T19:20:30.123Z"));
        assert!(!is_datetime("2023-07-16 19:20:30"));
        assert!(!is_datetime("not a date"));
    }

    #[test]
    fn test_classify_text_email() {
        assert_eq!(classify_text("user@example.com"), TextClass::Email);
        assert_eq!(classify_text("john.doe@sub.domain.co.uk"), TextClass::Email);
        assert_ne!(classify_text("not-an-email"), TextClass::Email);
    }

    #[test]
    fn test_classify_text_url() {
        assert_eq!(classify_text("https://example.com"), TextClass::Url);
        assert_eq!(classify_text("http://example.com/path"), TextClass::Url);
        assert_eq!(classify_text("example.com"), TextClass::Url);
        assert_ne!(classify_text("not-a-url"), TextClass::Url);
    }

    #[test]
    fn test_classify_text_numeric() {
        assert_eq!(classify_text("123456"), TextClass::Numeric);
        assert_ne!(classify_text("123abc"), TextClass::Numeric);
    }

    #[test]
    fn test_classify_text_alpha() {
        assert_eq!(classify_text("abcdef"), TextClass::Alpha);
        assert_eq!(classify_text("XYZ"), TextClass::Alpha);
        assert_ne!(classify_text("abc123"), TextClass::Alpha);
    }

    #[test]
    fn test_classify_text_alphanumeric() {
        assert_eq!(classify_text("abc123"), TextClass::Alphanumeric);
        assert_eq!(classify_text("A1B2C3"), TextClass::Alphanumeric);
        assert_ne!(classify_text("abc-123"), TextClass::Alphanumeric);
    }

    #[test]
    fn test_classify_text_other() {
        assert_eq!(classify_text("abc-123"), TextClass::Other);
        assert_eq!(classify_text("!@#$%^&*()"), TextClass::Other);
    }
}