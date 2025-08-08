use chrono::{DateTime, TimeZone, Utc};
use data_encoding::BASE32_NOPAD;
use lazy_static::lazy_static;
use rand::Rng;
use sha3::{Digest, Sha3_256};
use std::sync::Mutex;

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

/// Detects whether the provided string is a lowercase Base32 value. The string
/// must consist only of the characters `a`-`z` and `2`-`7` and have a length
/// greater than 16 characters.
pub fn is_base32_lowercase(input: &str) -> bool {
    input.len() > 16 && input.chars().all(|c| matches!(c, 'a'..='z' | '2'..='7'))
}

/// Detects whether the provided string is an uppercase Base32 value. The
/// string must consist only of the characters `A`-`Z` and `2`-`7` and have a
/// length greater than 16 characters.
pub fn is_base32_uppercase(input: &str) -> bool {
    input.len() > 16 && input.chars().all(|c| matches!(c, 'A'..='Z' | '2'..='7'))
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

/// Obfuscate a lowercase Base32 string by hashing it with SHA3-256 and encoding
/// the hash using lowercase Base32 without padding. The resulting string is
/// truncated or repeated so that its length matches the input.
pub fn obfuscate_base32_lowercase(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    let encoded = BASE32_NOPAD.encode(hash.as_ref()).to_lowercase();
    if encoded.len() >= input.len() {
        encoded[..input.len()].to_string()
    } else {
        let mut out = String::with_capacity(input.len());
        let mut iter = encoded.chars().cycle();
        while out.len() < input.len() {
            if let Some(ch) = iter.next() {
                out.push(ch);
            }
        }
        out
    }
}

/// Obfuscate an uppercase Base32 string by hashing it with SHA3-256 and encoding
/// the hash using uppercase Base32 without padding. The resulting string is
/// truncated or repeated so that its length matches the input.
pub fn obfuscate_base32_uppercase(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize();

    let encoded = BASE32_NOPAD.encode(hash.as_ref()).to_uppercase();
    if encoded.len() >= input.len() {
        encoded[..input.len()].to_string()
    } else {
        let mut out = String::with_capacity(input.len());
        let mut iter = encoded.chars().cycle();
        while out.len() < input.len() {
            if let Some(ch) = iter.next() {
                out.push(ch);
            }
        }
        out
    }
}

fn random_date_between_1970_and_now() -> DateTime<Utc> {
    let end = Utc::now().timestamp();
    let mut rng = rand::thread_rng();
    let secs = rng.gen_range(0..=end);
    Utc.timestamp_opt(secs, 0).single().unwrap()
}

lazy_static! {
    static ref NEW_DATE_BASELINE: Mutex<DateTime<Utc>> =
        Mutex::new(random_date_between_1970_and_now());
    static ref ORIGINAL_DATE_BASELINE: Mutex<Option<DateTime<Utc>>> = Mutex::new(None);
}

#[cfg(test)]
lazy_static! {
    pub static ref DATE_TEST_GUARD: Mutex<()> = Mutex::new(());
}

#[cfg(test)]
pub fn set_date_baselines(new_base: DateTime<Utc>) {
    let mut new_lock = NEW_DATE_BASELINE.lock().unwrap();
    *new_lock = new_base;
    let mut orig_lock = ORIGINAL_DATE_BASELINE.lock().unwrap();
    *orig_lock = None;
}

/// Detects whether the provided string is an ISO 8601 datetime with a trailing
/// `Z` designator.
pub fn is_iso8601_z_datetime(input: &str) -> bool {
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        if input.ends_with('Z') {
            dt.with_timezone(&Utc)
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string()
                == input
        } else {
            false
        }
    } else {
        false
    }
}

/// Obfuscate an ISO 8601 `Z` datetime by shifting it relative to runtime
/// baselines. The resulting value remains a valid ISO 8601 `Z` datetime.
pub fn obfuscate_iso8601_z_datetime(input: &str) -> String {
    let dt = DateTime::parse_from_rfc3339(input)
        .expect("invalid datetime")
        .with_timezone(&Utc);
    let mut orig = ORIGINAL_DATE_BASELINE.lock().unwrap();
    let orig_dt = match *orig {
        Some(ref orig_dt) => orig_dt.clone(),
        None => {
            *orig = Some(dt);
            dt
        }
    };
    let delta = orig_dt - dt;
    let new_dt_base = NEW_DATE_BASELINE.lock().unwrap().clone();
    let new_dt = new_dt_base + delta;
    new_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use std::collections::BTreeSet;

    include!("../tests/well_known_inputs.rs");

    fn reset_date_baselines() {
        super::set_date_baselines(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap());
    }

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
    fn test_is_base32_lowercase_examples() {
        assert!(is_base32_lowercase("mfrggzdfmztwq2lknnwg23tp"));
        assert!(!is_base32_lowercase("MFRGGZDFMZTWQ2LKNNWG23TP"));
        assert!(!is_base32_lowercase("mfrggzdfmztwq2lk"));
        assert!(!is_base32_lowercase("mfrggzdfmztwq2lk!!"));
    }

    #[test]
    fn test_obfuscate_base32_lowercase_preserves_class() {
        let value = "mfrggzdfmztwq2lknnwg23tp";
        let obf = obfuscate_base32_lowercase(value);
        assert!(is_base32_lowercase(&obf));
        assert_eq!(obf.len(), value.len());
    }

    #[test]
    fn test_is_base32_uppercase_examples() {
        assert!(is_base32_uppercase("MFRGGZDFMZTWQ2LKNNWG23TP"));
        assert!(!is_base32_uppercase("mfrggzdfmztwq2lknnwg23tp"));
        assert!(!is_base32_uppercase("MFRGGZDFMZTWQ2LK!!"));
    }

    #[test]
    fn test_obfuscate_base32_uppercase_preserves_class() {
        let value = "MFRGGZDFMZTWQ2LKNNWG23TP";
        let obf = obfuscate_base32_uppercase(value);
        assert!(is_base32_uppercase(&obf));
        assert!(!is_base32_lowercase(&obf));
        assert_eq!(obf.len(), value.len());
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
            if is_iso8601_z_datetime(example.input) {
                detected.insert("iso8601_z_datetime");
            }
            if is_base32_lowercase(example.input) {
                detected.insert("base32_lowercase");
            }
            if is_base32_uppercase(example.input) {
                detected.insert("base32_uppercase");
            }
            let expected: BTreeSet<&str> = example.detectors.iter().copied().collect();
            assert_eq!(detected, expected, "mismatch for input: {}", example.input);
        }
    }

    #[test]
    fn test_well_known_inputs_obfuscation() {
        let _guard = super::DATE_TEST_GUARD.lock().unwrap();
        reset_date_baselines();
        for example in WELL_KNOWN_INPUTS {
            for &name in example.detectors {
                let obf = match name {
                    "alpha_word" => hash_word_to_syllables(example.input),
                    "uppercase_word" => obfuscate_uppercase_word(example.input),
                    "capitalized_word" => obfuscate_capitalized_word(example.input),
                    "snake_case_word" => obfuscate_snake_case_word(example.input),
                    "title_case_sentence" => obfuscate_title_case_sentence(example.input),
                    "iso8601_z_datetime" => obfuscate_iso8601_z_datetime(example.input),
                    "base32_lowercase" => obfuscate_base32_lowercase(example.input),
                    "base32_uppercase" => obfuscate_base32_uppercase(example.input),
                    _ => continue,
                };
                let valid = match name {
                    "alpha_word" => is_alpha_word(&obf),
                    "uppercase_word" => is_uppercase_word(&obf),
                    "capitalized_word" => is_capitalized_word(&obf),
                    "snake_case_word" => is_snake_case_word(&obf),
                    "title_case_sentence" => is_title_case_sentence(&obf),
                    "iso8601_z_datetime" => is_iso8601_z_datetime(&obf),
                    "base32_lowercase" => is_base32_lowercase(&obf),
                    "base32_uppercase" => is_base32_uppercase(&obf),
                    _ => false,
                };
                assert!(valid, "{} obfuscation failed for {}", name, example.input);
            }
        }
    }

    #[test]
    fn test_is_iso8601_z_datetime_examples() {
        assert!(is_iso8601_z_datetime("2022-05-16T22:39:20Z"));
        assert!(!is_iso8601_z_datetime("2022-05-16T22:39:20+02:00"));
        assert!(!is_iso8601_z_datetime("2022-05-16"));
        assert!(!is_iso8601_z_datetime("not-a-date"));
    }

    #[test]
    fn test_obfuscate_iso8601_z_datetime_preserves_class() {
        let _guard = super::DATE_TEST_GUARD.lock().unwrap();
        reset_date_baselines();
        let first = "2022-05-16T22:39:20Z";
        let second = "2022-05-15T22:39:20Z";
        let obf_first = obfuscate_iso8601_z_datetime(first);
        let obf_second = obfuscate_iso8601_z_datetime(second);
        assert!(is_iso8601_z_datetime(&obf_first));
        assert!(is_iso8601_z_datetime(&obf_second));
        let obf_first_dt = chrono::DateTime::parse_from_rfc3339(&obf_first)
            .unwrap()
            .with_timezone(&Utc);
        let obf_second_dt = chrono::DateTime::parse_from_rfc3339(&obf_second)
            .unwrap()
            .with_timezone(&Utc);
        let first_dt = chrono::DateTime::parse_from_rfc3339(first)
            .unwrap()
            .with_timezone(&Utc);
        let second_dt = chrono::DateTime::parse_from_rfc3339(second)
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(obf_first_dt, *NEW_DATE_BASELINE.lock().unwrap());
        assert_eq!(obf_second_dt - obf_first_dt, first_dt - second_dt);
    }
}
