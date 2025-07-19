use sha3::{Digest, Sha3_256};

/// Detects whether the provided string is composed entirely of ASCII lowercase
/// letters.
///
/// This follows the repository's classifier conventions by using the `try_`
/// prefix and returning an `Option` containing the original string on success.
pub fn try_alpha_word(input: &str) -> Option<&str> {
    if !input.is_empty() && input.chars().all(|c| c.is_ascii_lowercase()) {
        Some(input)
    } else {
        None
    }
}

/// Detects whether the provided string is composed entirely of ASCII uppercase
/// letters.
///
/// The function follows the repository's classifier conventions by using the
/// `try_` prefix and returning an `Option` containing the original string on
/// success.
pub fn try_uppercase_word(input: &str) -> Option<&str> {
    if !input.is_empty() && input.chars().all(|c| c.is_ascii_uppercase()) {
        Some(input)
    } else {
        None
    }
}

/// Obfuscate an uppercase word into another deterministic uppercase word of the
/// same length. The output will also be recognised by `try_uppercase_word`.
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
    fn test_try_alpha_word_examples() {
        assert!(try_alpha_word("lowercase").is_some());
        assert!(try_alpha_word("lower_case").is_none());
        assert!(try_alpha_word("Lowercase").is_none());
        assert!(try_alpha_word("lower-case").is_none());
        assert!(try_alpha_word("LOWERCASE").is_none());
    }

    #[test]
    fn test_try_uppercase_word_examples() {
        assert!(try_uppercase_word("UPPERCASE").is_some());
        assert!(try_uppercase_word("UPPER_CASE").is_none());
        assert!(try_uppercase_word("UPPER CASE").is_none());
        assert!(try_uppercase_word("UPPER-CASE").is_none());
        assert!(try_uppercase_word("UPPERCASe").is_none());
    }

    #[test]
    fn test_obfuscate_uppercase_word_preserves_class() {
        let word = "SECRET";
        let obf = obfuscate_uppercase_word(word);
        assert_eq!(obf.len(), word.len());
        assert!(try_uppercase_word(&obf).is_some());
    }
}
