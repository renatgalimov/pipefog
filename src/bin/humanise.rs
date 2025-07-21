#[path = "../classifiers.rs"]
mod classifiers;
use std::io::{self, Read};
use classifiers::SYLLABLES;

fn bytes_to_syllables(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        out.push_str(SYLLABLES[b as usize]);
    }
    out
}

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let s = String::from_utf8_lossy(&buf);
    let trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = if !trimmed.is_empty()
        && trimmed.len() % 2 == 0
        && trimmed.chars().all(|c| c.is_ascii_hexdigit())
    {
        match hex::decode(&trimmed) {
            Ok(v) => v,
            Err(_) => buf,
        }
    } else {
        buf
    };

    let humanised = bytes_to_syllables(&bytes);
    println!("{}", humanised);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_syllables_simple() {
        let result = bytes_to_syllables(&[0x00, 0xff, 0x10]);
        assert_eq!(
            result,
            format!("{}{}{}", SYLLABLES[0], SYLLABLES[255], SYLLABLES[16])
        );
    }

    #[test]
    fn test_hex_input() {
        let input = b"0a0b";
        let s = String::from_utf8_lossy(input);
        let trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let bytes = if trimmed.len() % 2 == 0 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            hex::decode(&trimmed).unwrap()
        } else {
            input.to_vec()
        };
        assert_eq!(bytes, vec![0x0a, 0x0b]);
        assert_eq!(
            bytes_to_syllables(&bytes),
            format!("{}{}", SYLLABLES[0x0a], SYLLABLES[0x0b])
        );
    }
}

