use sha3::{Digest, Sha3_256};

/// Detects whether the provided string is composed entirely of ASCII uppercase
/// letters.
///
/// The function takes an `Option<&str>` rather than `&str` to align with the
/// repository's classifier conventions. If the input matches, the original
/// string is returned inside `Some`.
pub fn is_uppercase_word(value: Option<&str>) -> Option<&str> {
    value.filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_uppercase()))
}

/// Obfuscate an uppercase word into another deterministic uppercase word of the same length.
/// The output will also be recognised by `is_uppercase_word`.
pub fn obfuscate_uppercase_word(word: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(word.as_bytes());
    let hash = hasher.finalize();
    let mut out = String::with_capacity(word.len());
    while out.len() < word.len() {
        for &b in hash.as_slice() {
            let c = ((b % 26) as u8 + b'A') as char;
            out.push(c);
            if out.len() >= word.len() {
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_uppercase_word_examples() {
        assert!(is_uppercase_word(Some("UPPERCASE")).is_some());
        assert!(is_uppercase_word(Some("UPPER_CASE")).is_none());
        assert!(is_uppercase_word(Some("UPPER CASE")).is_none());
        assert!(is_uppercase_word(Some("UPPER-CASE")).is_none());
        assert!(is_uppercase_word(Some("UPPERCASe")).is_none());
        assert!(is_uppercase_word(None).is_none());
    }

    #[test]
    fn test_obfuscate_uppercase_word_preserves_class() {
        let word = "SECRET";
        let obf = obfuscate_uppercase_word(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_uppercase_word(Some(&obf)).is_some());
    }
}
