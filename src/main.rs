use sha3::{Digest, Sha3_256};
use serde_json::{Deserializer, Value};
use std::io::{self, Write};

fn hash_strings(value: &mut Value) {
    match value {
        Value::String(s) => {
            let mut hasher = Sha3_256::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            *s = hex::encode(result);
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                hash_strings(v);
            }
        }
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                hash_strings(v);
            }
        }
        _ => {}
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = io::BufWriter::new(handle);

    for value in stream {
        match value {
            Ok(mut val) => {
                hash_strings(&mut val);
                serde_json::to_writer_pretty(&mut writer, &val).expect("write json");
                writer.write_all(b"\n").expect("newline");
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sha3_hex(input: &str) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    #[test]
    fn test_hash_strings_simple() {
        let mut value = json!({
            "a": "test",
            "b": ["x", 1],
            "c": {"d": "y"}
        });

        hash_strings(&mut value);

        assert_eq!(value["a"], json!(sha3_hex("test")));
        assert_eq!(value["b"][0], json!(sha3_hex("x")));
        assert_eq!(value["b"][1], json!(1));
        assert_eq!(value["c"]["d"], json!(sha3_hex("y")));
    }
}
