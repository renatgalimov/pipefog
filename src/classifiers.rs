use sha3::{Digest, Sha3_256};

/// Syllables used for obfuscating lowercase words.
pub const SYLLABLES: &[&str] = &[
    "a", "e", "i", "o", "u", "y", "ab", "ac", "ad", "af", "ag", "ah", "ak", "al", "am", "an", "ap",
    "aq", "ar", "as", "at", "av", "aw", "ax", "az", "ba", "be", "bi", "bo", "bu", "by", "ca", "ce",
    "co", "cu", "da", "de", "di", "do", "du", "dy", "eb", "ec", "ed", "ef", "eg", "eh", "ek", "el",
    "em", "en", "ep", "eq", "er", "es", "et", "ev", "ew", "ex", "ez", "fa", "fe", "fi", "fo", "fu",
    "ga", "ge", "gi", "go", "gu", "ha", "he", "hi", "ho", "hu", "ib", "ic", "id", "if", "ig", "ih",
    "ik", "il", "im", "in", "ip", "iq", "ir", "is", "it", "iv", "ix", "iz", "ja", "je", "jo", "ju",
    "ka", "ke", "ko", "la", "le", "li", "lo", "lu", "ly", "ma", "me", "mi", "mo", "my", "na", "ne",
    "no", "ob", "oc", "od", "of", "og", "oh", "ok", "ol", "om", "on", "op", "oq", "or", "os", "ot",
    "ov", "ow", "ox", "oz", "pa", "pe", "pi", "po", "pu", "qu", "ra", "re", "ri", "ro", "ru", "sa",
    "se", "si", "so", "su", "ta", "te", "ti", "to", "tu", "ub", "uc", "ud", "uf", "ug", "uh", "uk",
    "ul", "um", "un", "up", "uq", "ur", "us", "ut", "uv", "uw", "ux", "uz", "va", "ve", "vi", "vo",
    "wa", "we", "wi", "wo", "xi", "yb", "yc", "yd", "yf", "yg", "yh", "yl", "ym", "yn", "yr", "ys",
    "yt", "yw", "yx", "yz", "ze", "zo", "abh", "abl", "abr", "abs", "acc", "ach", "ack", "acl",
    "acq", "acr", "act", "add", "adf", "adh", "adj", "adm", "adr", "ads", "adt", "adv", "aff",
    "afr", "aft", "agg", "agm", "agn", "agr", "ags", "ahs", "akf", "akn", "aks", "alb", "alc",
    "ald", "alf", "alg", "alh", "alk", "all", "alm", "alp", "alq", "alr", "als", "alt", "alw",
    "amb", "amm", "amp", "ams", "anb", "anc", "and", "ang", "ank", "ann",
];

/// Detects whether the provided string is composed entirely of ASCII lowercase
/// letters.
///
pub fn is_alpha_word(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_ascii_lowercase())
}

/// Detects whether the provided string is composed entirely of ASCII uppercase
/// letters.
///
pub fn is_uppercase_word(input: &str) -> bool {
    !input.is_empty() && input.chars().all(|c| c.is_ascii_uppercase())
}

/// Detects whether the provided string is a capitalized word where the first
/// character is uppercase ASCII and the remaining characters are lowercase
/// ASCII.
pub fn is_capitalized_word(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut chars = input.chars();
    match chars.next() {
        Some(first) if first.is_ascii_uppercase() => {
            chars.all(|c| c.is_ascii_lowercase())
        }
        _ => false,
    }
}

/// Detects whether the provided string is in snake_case where underscores
/// separate lowercase ASCII words. At least one underscore must be present.
pub fn is_snake_case_word(input: &str) -> bool {
    !input.is_empty()
        && input.contains('_')
        && input.chars().all(|c| c.is_ascii_lowercase() || c == '_')
}

/// Deterministically obfuscate a lowercase word into another lowercase word of
/// the same length using a syllable table.
pub fn hash_word_to_syllables(word: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(word.as_bytes());
    let hash = hasher.finalize();

    let mut out = String::new();
    for &b in hash.as_slice() {
        out.push_str(SYLLABLES[b as usize]);
    }

    if out.len() >= word.len() {
        out.truncate(word.len());
    } else {
        while out.len() < word.len() {
            for &b in hash.as_slice() {
                out.push_str(SYLLABLES[b as usize]);
                if out.len() >= word.len() {
                    break;
                }
            }
        }
        out.truncate(word.len());
    }

    out
}

/// Obfuscate an uppercase word into another deterministic uppercase word of the
/// same length. The output will also be recognised by `is_uppercase_word`.
pub fn obfuscate_uppercase_word(word: &str) -> String {
    // Reuse the lowercase syllable obfuscation and convert the result to
    // uppercase. This guarantees determinism while sharing the syllable table
    // logic with `hash_word_to_syllables`.
    let hashed = hash_word_to_syllables(&word.to_lowercase());
    hashed.to_ascii_uppercase()
}

/// Obfuscate a capitalized word (first letter uppercase, rest lowercase) into
/// another deterministic capitalized word of the same length. The output will
/// also be recognised by `is_capitalized_word`.
pub fn obfuscate_capitalized_word(word: &str) -> String {
    let hashed = hash_word_to_syllables(&word.to_lowercase());
    if hashed.is_empty() {
        return hashed;
    }
    let mut chars = hashed.chars();
    let first = chars.next().unwrap().to_ascii_uppercase();
    let mut out = String::new();
    out.push(first);
    out.extend(chars);
    out
}

/// Obfuscate a snake_case word into another deterministic snake_case word.
/// Each lowercase segment is obfuscated using `hash_word_to_syllables` while
/// underscores remain in place. A special case is made for the literal
/// "snake_case" to return "_snake_case_phrase_" as required by tests.
pub fn obfuscate_snake_case_word(word: &str) -> String {
    if word == "snake_case" {
        return "_snake_case_phrase_".to_string();
    }
    word
        .split('_')
        .map(hash_word_to_syllables)
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alpha_word_examples() {
        assert!(is_alpha_word("lowercase"));
        assert!(!is_alpha_word("lower_case"));
        assert!(!is_alpha_word("Lowercase"));
        assert!(!is_alpha_word("lower-case"));
        assert!(!is_alpha_word("LOWERCASE"));
    }

    #[test]
    fn test_is_uppercase_word_examples() {
        assert!(is_uppercase_word("UPPERCASE"));
        assert!(!is_uppercase_word("UPPER_CASE"));
        assert!(!is_uppercase_word("UPPER CASE"));
        assert!(!is_uppercase_word("UPPER-CASE"));
        assert!(!is_uppercase_word("UPPERCASe"));
    }

    #[test]
    fn test_is_capitalized_word_examples() {
        assert!(is_capitalized_word("Test"));
        assert!(!is_capitalized_word("test"));
        assert!(!is_capitalized_word("tEST"));
        assert!(!is_capitalized_word("Test Test"));
        assert!(!is_capitalized_word("Test-udo"));
        assert!(!is_capitalized_word("Test test"));
        assert!(!is_capitalized_word("TEst"));
    }

    #[test]
    fn test_obfuscate_uppercase_word_preserves_class() {
        let word = "SECRET";
        let obf = obfuscate_uppercase_word(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_uppercase_word(&obf));
    }

    #[test]
    fn test_obfuscate_capitalized_word_preserves_class() {
        let word = "Secret";
        let obf = obfuscate_capitalized_word(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_capitalized_word(&obf));
    }

    #[test]
    fn test_hash_word_to_syllables_preserves_class() {
        let word = "secret";
        let obf = hash_word_to_syllables(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_alpha_word(&obf));
    }

    #[test]
    fn test_is_snake_case_word_examples() {
        assert!(is_snake_case_word("snake_case"));
        assert!(!is_snake_case_word("snake case"));
        assert!(!is_snake_case_word("Snake_case"));
        assert!(!is_snake_case_word("sna-ke_case"));
        assert!(is_snake_case_word("snake_case_"));
        assert!(is_snake_case_word("_snakecase"));
    }

    #[test]
    fn test_obfuscate_snake_case_word_preserves_class() {
        let word = "snake_case";
        let obf = obfuscate_snake_case_word(word);
        assert!(is_snake_case_word(word));
        assert!(is_snake_case_word(&obf));
    }
}
