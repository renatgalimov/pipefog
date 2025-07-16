use human_hash::HumanHasher;
use regex::Regex;
use serde_json::{Deserializer, Value};
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::io::{self, BufRead, Write};
use uuid::Uuid;

/// Checks if a string is in ISO8601 datetime format.
pub fn is_datetime(s: &str) -> bool {
    // Basic ISO8601 regex: 2023-07-16T19:20:30Z or 2023-07-16T19:20:30+01:00
    // This is a simplified version and may be adjusted for stricter matching
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})$").unwrap();
    re.is_match(s)
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
            let mut hasher = DefaultHasher::new();
            let word_count = s.split_whitespace().count();
            hasher.write(s.as_bytes());
            let hash = hasher.finish();
            let human_hash = HumanHasher::new(human_hash::DEFAULT_WORDLIST)
                .humanize(&Uuid::from_u64_pair(hash, hash.wrapping_add(1)), word_count)
                .replace("-", " ");
            *s = human_hash.to_string();
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
    use super::is_datetime;

    #[test]
    fn test_is_datetime() {
        assert!(is_datetime("2023-07-16T19:20:30Z"));
        assert!(is_datetime("2023-07-16T19:20:30+01:00"));
        assert!(is_datetime("2023-07-16T19:20:30-05:00"));
        assert!(is_datetime("2023-07-16T19:20:30.123Z"));
        assert!(!is_datetime("2023-07-16 19:20:30"));
        assert!(!is_datetime("not a date"));
    }
}