use sha3::{Digest, Sha3_256};

/// Syllables used for obfuscating lowercase words.
pub const SYLLABLES: &[&str] = &[
    "a", "e", "i", "o", "u", "y", "ab", "ac", "ad", "af", "ag", "ah", "ak", "al", "am", "an", "ap",
    "aq", "ar", "as", "at", "av", "aw", "ax", "az", "ba", "be", "bi", "bo", "bu", "by", "ca", "ce",
    "co", "cu", "da", "de", "di", "do", "du", "dy", "eb", "ec", "ed", "ef", "eg", "eh", "ek", "el",
    "em", "en", "ep", "eq", "er", "es", "et", "ev", "ew", "ex", "ez", "fa", "fe", "fi", "fo", "fu", "ga",
    "ge", "gi", "go", "gu", "ha", "he", "hi", "ho", "hu", "ib", "ic", "id", "if", "ig", "ih", "ik", "il",
    "im", "in", "ip", "iq", "ir", "is", "it", "iv", "ix", "iz", "ja", "je", "jo", "ju", "ka", "ke", "ko", "la",
    "le", "li", "lo", "lu", "ly", "ma", "me", "mi", "mo", "my", "na", "ne", "no", "ob", "oc", "od", "of", "og",
    "oh", "ok", "ol", "om", "on", "op", "oq", "or", "os", "ot", "ov", "ow", "ox", "oz", "pa", "pe", "pi", "po",
    "pu", "qu", "ra", "re", "ri", "ro", "ru", "sa", "se", "si", "so", "su", "ta", "te", "ti", "to", "tu", "ub", "uc",
    "ud", "uf", "ug", "uh", "uk", "ul", "um", "un", "up", "uq", "ur", "us", "ut", "uv", "uw", "ux", "uz", "va", "ve",
    "vi", "vo", "wa", "we", "wi", "wo", "xi", "yb", "yc", "yd", "yf", "yg", "yh", "yl", "ym", "yn", "yr", "ys", "yt",
    "yw", "yx", "yz", "ze", "zo", "abh", "abl", "abr", "abs", "acc", "ach", "ack", "acl", "acq", "acr", "act", "add",
    "adf", "adh", "adj", "adm", "adr", "ads", "adt", "adv", "aff", "afr", "aft", "agg", "agm", "agn", "agr", "ags",
    "ahs", "akf", "akn", "aks", "alb", "alc", "ald", "alf", "alg", "alh", "alk", "all", "alm", "alp", "alq", "alr",
    "als", "alt", "alw", "amb", "amm", "amp", "ams", "anb", "anc", "and", "ang", "ank", "ann",
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
    let mut hasher = Sha3_256::new();
    hasher.update(word.as_bytes());
    let hash = hasher.finalize();
    let mut out = String::new();

    for &b in hash.as_slice() {
        let c = ((b % 26) as u8 + b'A') as char;
        out.push(c);
    }

    if out.len() >= word.len() {
        out.truncate(word.len());
    } else {
        while out.len() < word.len() {
            for &b in hash.as_slice() {
                let c = ((b % 26) as u8 + b'A') as char;
                out.push(c);
                if out.len() >= word.len() {
                    break;
                }
            }
        }
        out.truncate(word.len());
    }

    out
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
    fn test_obfuscate_uppercase_word_preserves_class() {
        let word = "SECRET";
        let obf = obfuscate_uppercase_word(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_uppercase_word(&obf));
    }

    #[test]
    fn test_hash_word_to_syllables_preserves_class() {
        let word = "secret";
        let obf = hash_word_to_syllables(word);
        assert_eq!(obf.len(), word.len());
        assert!(is_alpha_word(&obf));
    }
}
