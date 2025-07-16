use serde_json::{Deserializer, Value};
use std::io::{BufReader, BufWriter, Cursor, Write};

fn process_json_stream(input: &str) -> String {
    let mut output = Vec::new();
    {
        let mut writer = BufWriter::new(&mut output);
        for line in input.lines() {
            match serde_json::from_str::<Value>(line) {
                Ok(val) => {
                    serde_json::to_writer(&mut writer, &val).expect("Failed to write JSON");
                    writer.write_all(b"\n").expect("Failed to write newline");
                }
                Err(_) => {
                    // skip invalid JSON lines
                    continue;
                }
            }
        }
    }
    String::from_utf8(output).unwrap()
}

#[cfg(test)]
mod tests {
    use super::process_json_stream;

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
