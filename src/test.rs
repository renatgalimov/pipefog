#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::{Deserializer, Value};
    use std::io::{BufReader, BufWriter, Cursor};

    fn obfuscate_json_value(value: &mut Value) {
        match value {
            Value::Array(arr) => {
                for v in arr.iter_mut() {
                    obfuscate_json_value(v);
                }
            }
            Value::Object(map) => {
                for v in map.values_mut() {
                    obfuscate_json_value(v);
                }
            }
            Value::String(s) => {
                let mut hasher = Hasher::new();
                hasher.write_str(&s);
                let hash = hasher.finish();
                
                let obfuscated = human_hash::human_hash(s);
                *s = obfuscated;
            }
            _ => {}
        }
    }

    fn process_json_stream(input: &str) -> String {
        let reader = BufReader::new(Cursor::new(input));
        let mut output = Vec::new();
        let mut writer = BufWriter::new(&mut output);
        let stream = Deserializer::from_reader(reader).into_iter::<Value>();
        for value in stream {
            match value {
                Ok(mut val) => {
                    obfuscate_json_value(&mut val);
                    serde_json::to_writer(&mut writer, &val).expect("Failed to write JSON");
                    writer.write_all(b"\n").expect("Failed to write newline");
                }
                Err(_) => {}
            }
        }
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn test_single_json_object() {
        let input = "{\"a\":1}\n";
        let output = process_json_stream(input);
        assert_eq!(output, "{\"a\":1}\n");
    }

    #[test]
    fn test_multiple_json_objects() {
        let input = "{\"a\":1}\n{\"b\":2}\n";
        let output = process_json_stream(input);
        assert_eq!(output, "{\"a\":1}\n{\"b\":2}\n");
    }

    #[test]
    fn test_invalid_json() {
        let input = "{\"a\":1}\ninvalid\n{\"b\":2}\n";
        let output = process_json_stream(input);
        assert_eq!(output, "{\"a\":1}\n{\"b\":2}\n");
    }
}
