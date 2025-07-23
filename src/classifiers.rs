use sha3::{Digest, Sha3_256};

/// Syllables used for obfuscating lowercase words.
pub const SYLLABLES: &[&str] = &[
    "plac", "most", "sam", "ke", "uth", "arl", "het", "giv", "fa", "first", "own", "li", "van",
    "form", "pres", "ond", "men", "bef", "old", "agr", "must", "two", "ight", "mak", "cons", "nat",
    "den", "rem", "inst", "eb", "itt", "iss", "tak", "ars", "ap", "app", "iz", "wher", "ec", "mad",
    "cont", "pe", "such", "lik", "ung", "rec", "gen", "now", "how", "urs", "wa", "ver", "than",
    "don", "com", "mo", "ught", "pa", "min", "vi", "comm", "sho", "thes", "ents", "then", "aft",
    "fe", "ek", "ha", "ins", "ep", "ich", "acc", "elf", "ans", "can", "ass", "att", "ni", "ex",
    "work", "par", "ef", "te", "part", "ho", "onl", "des", "vo", "tim", "ib", "lo", "has", "tho",
    "proj", "ert", "gre", "ord", "off", "stat", "what", "ort", "der", "eg", "gut", "ach", "art",
    "si", "ett", "ern", "als", "enb", "bo", "ud", "ys", "them", "som", "mor", "act", "unt", "who",
    "ac", "ak", "ik", "ish", "ast", "when", "erg", "po", "ne", "ard", "will", "go", "ugh", "ro",
    "um", "da", "ens", "ow", "ja", "my", "ind", "ok", "op", "wo", "anc", "ill", "abl", "ther",
    "fo", "she", "av", "him", "ot", "oth", "ig", "ov", "its", "ell", "wer", "enc", "ma", "man",
    "di", "od", "end", "do", "up", "re", "no", "im", "le", "ab", "om", "sa", "ul", "ant", "co",
    "if", "uld", "ist", "hav", "ons", "la", "we", "from", "me", "had", "but", "her", "which", "so",
    "ag", "int", "se", "est", "ol", "os", "qu", "un", "this", "ev", "ect", "ers", "iv", "em",
    "not", "am", "by", "ess", "und", "ad", "il", "his", "ir", "all", "for", "was", "id", "de",
    "with", "et", "that", "be", "ut", "ic", "us", "el", "ur", "he", "ent", "as", "or", "al", "ar",
    "is", "an", "u", "ing", "at", "it", "es", "to", "and", "en", "on", "of", "ed", "o", "in", "er",
    "i", "a", "y", "the", "e",
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
        Some(first) if first.is_ascii_uppercase() => chars.all(|c| c.is_ascii_lowercase()),
        _ => false,
    }
}

/// Detects whether the provided string is snake_case consisting of ASCII
/// lowercase letters and underscores with at least one underscore.
pub fn is_snake_case_word(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut has_underscore = false;
    for c in input.chars() {
        if c == '_' {
            has_underscore = true;
        } else if !c.is_ascii_lowercase() {
            return false;
        }
    }
    has_underscore
}

/// Detects whether the provided string is a sentence in Title Case. Each word
/// must start with a capital letter followed by lowercase letters. Single-letter
/// words must be uppercase.
pub fn is_title_case_sentence(input: &str) -> bool {
    if input.trim().is_empty() {
        return false;
    }
    let mut word_count = 0;
    for token in input.split_whitespace() {
        let trimmed = token.trim_matches(|c: char| !c.is_ascii_alphabetic());
        if trimmed.is_empty() {
            return false;
        }
        if trimmed.chars().count() == 1 {
            if !is_uppercase_word(trimmed) {
                return false;
            }
        } else if !is_capitalized_word(trimmed) {
            return false;
        }
        word_count += 1;
    }
    word_count > 1
}

/// Split a word into approximate English syllables using the same logic as the
/// helper in `bin/syllable_frequency.rs`.
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

/// Produce a deterministic vector of syllables for a word using the same
/// hashing mechanism as `hash_word_to_syllables`. The returned vector will
/// contain `count` syllables, repeating the hash output if necessary.
pub fn hash_word_to_syllable_vec(word: &str, count: usize) -> Vec<&'static str> {
    let mut hasher = Sha3_256::new();
    hasher.update(word.as_bytes());
    let hash = hasher.finalize();

    let mut out = Vec::with_capacity(count);
    let mut iter = hash.as_slice().iter().cycle();
    for _ in 0..count {
        if let Some(b) = iter.next() {
            out.push(SYLLABLES[*b as usize]);
        }
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

/// Obfuscate a snake_case word by hashing all characters except underscores.
/// The hashed syllables are combined in pairs and an underscore is inserted
/// between each pair. Leading and trailing underscores from the input are
/// preserved. The resulting string will still satisfy `is_snake_case_word`.
pub fn obfuscate_snake_case_word(word: &str) -> String {
    let leading = word.chars().take_while(|&c| c == '_').count();
    let trailing = word.chars().rev().take_while(|&c| c == '_').count();

    let letters: String = word.chars().filter(|&c| c != '_').collect();
    // Use the full word for hashing so that underscore positions affect the
    // result. Determine the number of output syllables based on a simple
    // English syllable split of the letters-only portion.
    let syllable_count = rough_english_syllables(&letters).len();
    let syllables = hash_word_to_syllable_vec(word, syllable_count);

    let mut parts = Vec::new();
    let mut i = 0;
    while i < syllables.len() {
        let mut part = String::new();
        part.push_str(syllables[i]);
        if i + 1 < syllables.len() {
            part.push_str(syllables[i + 1]);
        }
        parts.push(part);
        i += 2;
    }

    let core = parts.join("_");

    let mut out = String::new();
    out.extend(std::iter::repeat('_').take(leading));
    out.push_str(&core);
    out.extend(std::iter::repeat('_').take(trailing));
    out
}

/// Obfuscate a Title Case sentence by hashing the entire sentence and
/// rebuilding each word from the hash. The resulting sentence will still be in
/// Title Case.
pub fn obfuscate_title_case_sentence(sentence: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(sentence.as_bytes());
    let hash = hasher.finalize();
    let mut iter = hash.as_slice().iter().cycle();

    let mut out_words = Vec::new();
    for token in sentence.split_whitespace() {
        let trimmed = token.trim_matches(|c: char| !c.is_ascii_alphabetic());
        let start = token.find(trimmed).unwrap_or(0);
        let end = start + trimmed.len();
        let leading = &token[..start];
        let trailing = &token[end..];
        let mut word = String::new();
        while word.len() < trimmed.len() {
            if let Some(b) = iter.next() {
                word.push_str(SYLLABLES[*b as usize]);
            }
        }
        word.truncate(trimmed.len());
        let word = if trimmed.chars().count() == 1 {
            word.to_ascii_uppercase()
        } else {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                let mut tmp = String::new();
                tmp.push(first.to_ascii_uppercase());
                tmp.extend(chars);
                tmp
            } else {
                word
            }
        };
        let mut rebuilt = String::new();
        rebuilt.push_str(leading);
        rebuilt.push_str(&word);
        rebuilt.push_str(trailing);
        out_words.push(rebuilt);
    }

    out_words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    include!("../tests/well_known_inputs.rs");

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
        let word = "very_secret";
        let obf = obfuscate_snake_case_word(word);
        assert!(is_snake_case_word(&obf));
    }

    #[test]
    fn test_is_title_case_sentence_examples() {
        assert!(is_title_case_sentence("A Title Case Sentence"));
        assert!(!is_title_case_sentence("A title Case Sentence"));
        assert!(!is_title_case_sentence("A Title Case sentence"));
        assert!(!is_title_case_sentence("Capitalized"));
    }

    #[test]
    fn test_obfuscate_title_case_sentence_preserves_class() {
        let sentence = "A Title Case Sentence";
        let obf = obfuscate_title_case_sentence(sentence);
        assert!(is_title_case_sentence(&obf));
    }

    #[test]
    fn test_title_case_sentence_with_many_words() {
        let sentence = "An Example Of A Very Long Title Case Sentence That Contains Many Words And Continues For Quite Some Time Without Losing The Required Capitalization Pattern";
        assert!(is_title_case_sentence(sentence));
        let obf = obfuscate_title_case_sentence(sentence);
        assert!(is_title_case_sentence(&obf));
    }

    #[test]
    fn test_well_known_inputs_detection() {
        for example in WELL_KNOWN_INPUTS {
            let mut detected = BTreeSet::new();
            if is_alpha_word(example.input) {
                detected.insert("alpha_word");
            }
            if is_uppercase_word(example.input) {
                detected.insert("uppercase_word");
            }
            if is_capitalized_word(example.input) {
                detected.insert("capitalized_word");
            }
            if is_snake_case_word(example.input) {
                detected.insert("snake_case_word");
            }
            if is_title_case_sentence(example.input) {
                detected.insert("title_case_sentence");
            }
            let expected: BTreeSet<&str> = example.detectors.iter().copied().collect();
            assert_eq!(detected, expected, "mismatch for input: {}", example.input);
        }
    }

    #[test]
    fn test_well_known_inputs_obfuscation() {
        for example in WELL_KNOWN_INPUTS {
            for &name in example.detectors {
                let obf = match name {
                    "alpha_word" => hash_word_to_syllables(example.input),
                    "uppercase_word" => obfuscate_uppercase_word(example.input),
                    "capitalized_word" => obfuscate_capitalized_word(example.input),
                    "snake_case_word" => obfuscate_snake_case_word(example.input),
                    "title_case_sentence" => obfuscate_title_case_sentence(example.input),
                    _ => continue,
                };
                let valid = match name {
                    "alpha_word" => is_alpha_word(&obf),
                    "uppercase_word" => is_uppercase_word(&obf),
                    "capitalized_word" => is_capitalized_word(&obf),
                    "snake_case_word" => is_snake_case_word(&obf),
                    "title_case_sentence" => is_title_case_sentence(&obf),
                    _ => false,
                };
                assert!(valid, "{} obfuscation failed for {}", name, example.input);
            }
        }
    }
}
