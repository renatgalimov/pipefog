use serde_json::{Deserializer, Value};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let handle = stdout.lock();
    let reader = stdin.lock();
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();
    let mut writer = io::BufWriter::new(handle);

    for value in stream {
        match value {
            Ok(val) => {
                serde_json::to_writer(&mut writer, &val).expect("Failed to write JSON");
                writer.write_all(b"\n").expect("Failed to write newline");
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
}
