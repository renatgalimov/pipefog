use human_hash::HumanHasher;
use regex::Regex;
use serde_json::{Deserializer, Value};
use std::hash::{DefaultHasher, Hasher};
use std::io::{self, BufRead, Write};
use uuid::Uuid;

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
