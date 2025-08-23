use chrono::{DateTime, TimeZone, Utc};
use data_encoding::BASE32_NOPAD;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use serde_json::{Deserializer, Value};
use sha3::{Digest, Sha3_256};
use std::collections::HashMap;
use std::io::{self, Read, Write};
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
pub fn rough_english_syllables(word: &str) -> Vec<String> {
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
    let hash = Sha3_256::digest(word.as_bytes());
    hash_to_syllables(hash.as_slice(), word.len())
}

/// Produce a deterministic vector of syllables for a word using the same
/// hashing mechanism as `hash_word_to_syllables`. The returned vector will
/// contain `count` syllables, repeating the hash output if necessary.
pub fn hash_word_to_syllable_vec(word: &str, count: usize) -> Vec<&'static str> {
    let hash = Sha3_256::digest(word.as_bytes());
    hash_to_syllable_vec(hash.as_slice(), count)
}

/// Obfuscate an uppercase word into another deterministic uppercase word of the
/// same length. The output will also be recognised by `is_uppercase_word`.
pub fn obfuscate_uppercase_word(word: &str) -> String {
    let hash = Sha3_256::digest(word.as_bytes());
    hash_to_syllables(hash.as_slice(), word.len()).to_ascii_uppercase()
}

/// Obfuscate a capitalized word (first letter uppercase, rest lowercase) into
/// another deterministic capitalized word of the same length. The output will
/// also be recognised by `is_capitalized_word`.
pub fn obfuscate_capitalized_word(word: &str) -> String {
    let hash = Sha3_256::digest(word.as_bytes());
    let hashed = hash_to_syllables(hash.as_slice(), word.len());
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
    let hash = Sha3_256::digest(word.as_bytes());
    hash_to_snake_case(word, hash.as_slice())
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
    let hash = Sha3_256::digest(input.as_bytes());
    hash_to_base32_lowercase(hash.as_slice(), input.len())
}

/// Obfuscate an uppercase Base32 string by hashing it with SHA3-256 and encoding
/// the hash using uppercase Base32 without padding. The resulting string is
/// truncated or repeated so that its length matches the input.
pub fn obfuscate_base32_uppercase(input: &str) -> String {
    let hash = Sha3_256::digest(input.as_bytes());
    hash_to_base32_uppercase(hash.as_slice(), input.len())
}

fn random_date_between_1970_and_now() -> DateTime<Utc> {
    let end = Utc::now().timestamp();
    let mut rng = rand::thread_rng();
    let secs = rng.gen_range(0..=end);
    Utc.timestamp_opt(secs, 0).single().unwrap()
}

lazy_static! {
    pub static ref NEW_DATE_BASELINE: Mutex<DateTime<Utc>> =
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
        Some(ref orig_dt) => *orig_dt,
        None => {
            *orig = Some(dt);
            dt
        }
    };
    let delta = orig_dt - dt;
    let new_dt_base = *NEW_DATE_BASELINE.lock().unwrap();
    let new_dt = new_dt_base + delta;
    new_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

lazy_static! {
    static ref WORD_RE: Regex = Regex::new(r"[A-Za-z]+").unwrap();
}

fn hash_strings(value: &mut Value) {
    match value {
        Value::String(s) => {
            let hash = Sha3_256::digest(s.as_bytes());
            if is_alpha_word(s) {
                *s = hash_to_syllables(hash.as_slice(), s.len());
            } else if is_snake_case_word(s) {
                *s = hash_to_snake_case(s, hash.as_slice());
            } else if is_uppercase_word(s) {
                *s = hash_to_syllables(hash.as_slice(), s.len()).to_ascii_uppercase();
            } else if is_capitalized_word(s) {
                let hashed = hash_to_syllables(hash.as_slice(), s.len());
                if hashed.is_empty() {
                    *s = hashed;
                } else {
                    let mut chars = hashed.chars();
                    if let Some(first) = chars.next() {
                        *s = first.to_ascii_uppercase().to_string() + chars.as_str();
                    } else {
                        *s = hashed;
                    }
                }
            } else if is_iso8601_z_datetime(s) {
                *s = obfuscate_iso8601_z_datetime(s);
            } else if is_base32_uppercase(s) {
                *s = hash_to_base32_uppercase(hash.as_slice(), s.len());
            } else if is_base32_lowercase(s) {
                *s = hash_to_base32_lowercase(hash.as_slice(), s.len());
            } else {
                *s = hex::encode(hash.as_slice());
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                hash_strings(v);
            }
        }
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                hash_strings(v);
            }
        }
        _ => {}
    }
}

fn hash_to_syllables(hash: &[u8], len: usize) -> String {
    let mut out = String::new();
    for &b in hash {
        out.push_str(SYLLABLES[b as usize]);
    }
    if out.len() >= len {
        out.truncate(len);
    } else {
        while out.len() < len {
            for &b in hash {
                out.push_str(SYLLABLES[b as usize]);
                if out.len() >= len {
                    break;
                }
            }
        }
        out.truncate(len);
    }
    out
}

fn hash_to_syllable_vec(hash: &[u8], count: usize) -> Vec<&'static str> {
    let mut out = Vec::with_capacity(count);
    let mut iter = hash.iter().cycle();
    for _ in 0..count {
        if let Some(&b) = iter.next() {
            out.push(SYLLABLES[b as usize]);
        }
    }
    out
}

fn hash_to_snake_case(word: &str, hash: &[u8]) -> String {
    let leading = word.chars().take_while(|&c| c == '_').count();
    let trailing = word.chars().rev().take_while(|&c| c == '_').count();

    let letters: String = word.chars().filter(|&c| c != '_').collect();
    let syllable_count = rough_english_syllables(&letters).len();
    let syllables = hash_to_syllable_vec(hash, syllable_count);

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
    out.extend(std::iter::repeat_n('_', leading));
    out.push_str(&core);
    out.extend(std::iter::repeat_n('_', trailing));
    out
}

fn hash_to_base32_lowercase(hash: &[u8], len: usize) -> String {
    let encoded = BASE32_NOPAD.encode(hash).to_lowercase();
    if encoded.len() >= len {
        encoded[..len].to_string()
    } else {
        let mut out = String::with_capacity(len);
        let mut iter = encoded.chars().cycle();
        while out.len() < len {
            if let Some(ch) = iter.next() {
                out.push(ch);
            }
        }
        out
    }
}

fn hash_to_base32_uppercase(hash: &[u8], len: usize) -> String {
    let encoded = BASE32_NOPAD.encode(hash).to_uppercase();
    if encoded.len() >= len {
        encoded[..len].to_string()
    } else {
        let mut out = String::with_capacity(len);
        let mut iter = encoded.chars().cycle();
        while out.len() < len {
            if let Some(ch) = iter.next() {
                out.push(ch);
            }
        }
        out
    }
}

fn bytes_to_syllables(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        out.push_str(SYLLABLES[b as usize]);
    }
    out
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

fn default_mode() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = io::BufWriter::new(handle);

    for value in stream {
        match value {
            Ok(mut val) => {
                hash_strings(&mut val);
                serde_json::to_writer_pretty(&mut writer, &val)?;
                writer.write_all(b"\n")?;
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
    Ok(())
}

fn humanise() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;
    let s = String::from_utf8_lossy(&buf);
    let trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = if !trimmed.is_empty()
        && trimmed.len() % 2 == 0
        && trimmed.chars().all(|c| c.is_ascii_hexdigit())
    {
        hex::decode(&trimmed).unwrap_or(buf)
    } else {
        buf
    };
    let humanised = bytes_to_syllables(&bytes);
    println!("{}", humanised);
    Ok(())
}

fn syllable_frequency() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let vowels = "aeiouyAEIOUY";
    for (syl, count) in count_syllables(&input) {
        if !syl.chars().any(|c| vowels.contains(c)) {
            continue;
        }
        println!("{} {} {}", count, syl.len(), syl);
    }
    Ok(())
}

pub fn run() -> io::Result<()> {
    let mut args = std::env::args();
    let _ = args.next();
    match args.next().as_deref() {
        Some("humanise") => humanise(),
        Some("syllable-frequency") => syllable_frequency(),
        _ => default_mode(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json::json;
    use std::collections::BTreeSet;

    include!("../tests/well_known_inputs.rs");

    const TEST_SAMPLE: &str = r#"
        [
          {
            "id": "ehatv5afkscpijfhcdiwk2vgk5",
            "title": "Title",
            "lower case word": "lowercaseword",
            "version": 1,
            "vault": {
              "id": "ymmcavajzclbbyvnn6pmghw52n",
              "name": "Vaultname"
            },
            "category": "LOGIN",
            "last_edited_by": "LIS57PQOMZYK6YIAH6DN35JBCR",
            "created_at": "2025-05-07T11:58:32Z",
            "updated_at": "2025-05-07T11:58:32Z",
            "additional_information": "—",
            "urls": [
              {
                "label": "website",
                "primary": true,
                "href": "https://example.com/accounts/6d407c4c7578c31bdbe1dce529476c1a/signInPassword"
              }
            ]
          }
        ]"#;

    fn reset_date_baselines() {
        set_date_baselines(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap());
    }

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

    #[test]
    fn test_hash_strings_simple() {
        let mut value = json!({
            "a": "test",
            "b": ["x", 1],
            "c": {"d": "y"},
            "snake": "snake_case",
            "cap": "Word",
            "u": "UPPER",
            "b32u": "MFRGGZDFMZTWQ2LKNNWG23TP",
        });
        hash_strings(&mut value);
        assert_eq!(value["a"], json!("comi"));
        assert_eq!(value["b"][0], json!("s"));
        assert_eq!(value["b"][1], json!(1));
        assert_eq!(value["c"]["d"], json!("i"));
        assert_eq!(value["cap"], json!("Boge"));
        assert_eq!(value["snake"], json!("utcont_stathim"));
        assert_eq!(value["u"], json!("ERGIL"));
        assert_eq!(value["b32u"], json!("VLDMNPOCMVCVJCXFTLDUCL74"));
    }

    #[test]
    fn test_is_alpha_word_cases() {
        assert!(!is_alpha_word("Word"));
        assert!(is_alpha_word("word"));
        assert!(!is_alpha_word("wo_rd"));
        assert!(!is_alpha_word("wo-rd"));
        assert!(!is_alpha_word("WORD"));
    }

    #[test]
    fn test_hash_strings_test_sample() {
        let _guard = DATE_TEST_GUARD.lock().unwrap();
        set_date_baselines(Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap());
        let test_sample: Value =
            serde_json::from_str(TEST_SAMPLE).expect("Failed to parse TEST_SAMPLE");
        let mut hashed_sample = test_sample.clone();
        hash_strings(&mut hashed_sample);
        let hashes = serde_json::to_string_pretty(&hashed_sample)
            .expect("Failed to serialize hashed sample");
        const EXPECTED_HASHES: &str = r#"[
  {
    "additional_information": "4e9be9f98ffaf00dfa6849b118ec0eebaeb9d1fedf49794efc978549d692a644",
    "category": "AGRWH",
    "created_at": "2000-01-01T00:00:00Z",
    "id": "rdhx3wx7qo75n46jwl4n7wijq5",
    "last_edited_by": "S4XGJ7ZPIXFYST6VJC552D35IM",
    "lower case word": "vericthesneup",
    "title": "Danin",
    "updated_at": "2000-01-01T00:00:00Z",
    "urls": [
      {
        "href": "d0de71c6aff7c8a492c089fbd5a26a39e76716eef770728c5383386fc245c34b",
        "label": "enagwhi",
        "primary": true
      }
    ],
    "vault": {
      "id": "ynbhwzbd65ufp2foibsrlbv6js",
      "name": "Suchardwi"
    },
    "version": 1
  }
]"#;
        assert_eq!(hashes, EXPECTED_HASHES);
        let obj = hashed_sample.as_array().unwrap()[0].as_object().unwrap();
        let created = obj.get("created_at").unwrap().as_str().unwrap();
        assert!(is_iso8601_z_datetime(created));
        let updated = obj.get("updated_at").unwrap().as_str().unwrap();
        assert!(is_iso8601_z_datetime(updated));
        assert_eq!(created, updated);
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
    }

    #[test]
    fn test_is_snake_case_word_examples() {
        assert!(is_snake_case_word("snake_case"));
        assert!(!is_snake_case_word("snakecase"));
        assert!(!is_snake_case_word("Snake_Case"));
        assert!(!is_snake_case_word("snake-case"));
        assert!(!is_snake_case_word("snake case"));
    }

    #[test]
    fn test_is_title_case_sentence_examples() {
        assert!(is_title_case_sentence("A Title Case Sentence"));
        assert!(!is_title_case_sentence("A title Case"));
        assert!(!is_title_case_sentence("A"));
        assert!(!is_title_case_sentence(""));
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
        let _guard = DATE_TEST_GUARD.lock().unwrap();
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
        let _guard = DATE_TEST_GUARD.lock().unwrap();
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
