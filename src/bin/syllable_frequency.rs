use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn rough_english_syllables(word: &str) -> Vec<String> {
    let mut syllables = Vec::new();
    let mut buffer = String::new();
    let chars: Vec<char> = word.chars().collect();
    let vowels = "aeiouy";

    let mut i = 0;
    while i < chars.len() {
        buffer.push(chars[i]);

        if vowels.contains(chars[i]) {
            let mut j = i + 1;
            while j < chars.len() && !vowels.contains(chars[j]) {
                buffer.push(chars[j]);
                j += 1;
            }
            syllables.push(std::mem::take(&mut buffer));
            i = j;
        } else {
            i += 1;
        }
    }

    if !buffer.is_empty() {
        syllables.push(buffer);
    }

    syllables
}

lazy_static! {
    static ref WORD_RE: Regex = Regex::new(r"[A-Za-z]+").unwrap();
}

fn count_syllables(text: &str) -> Vec<(String, usize)> {
    let mut freq: HashMap<String, usize> = HashMap::new();

    for mat in WORD_RE.find_iter(text) {
        let word = mat.as_str().to_lowercase();
        for syl in rough_english_syllables(&word) {
            *freq.entry(syl).or_insert(0) += 1;
        }
    }

    let mut items: Vec<(String, usize)> = freq.into_iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    items
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    for (syl, count) in count_syllables(&input) {
        println!("{} {}", syl, count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_syllables_basic() {
        let result = count_syllables("Rust is amazing");
        let expected = vec![
            ("am".to_string(), 1),
            ("az".to_string(), 1),
            ("ing".to_string(), 1),
            ("is".to_string(), 1),
            ("rust".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_syllables_repeated() {
        let result = count_syllables("Hello world! Hello.");
        let expected = vec![
            ("hell".to_string(), 2),
            ("o".to_string(), 2),
            ("world".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }
}
