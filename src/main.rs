use serde_json::{Deserializer, Value};
use sha3::{Digest, Sha3_256};
use std::io::{self, Write};

mod classifiers;
use classifiers::{
    hash_word_to_syllables, is_alpha_word, is_uppercase_word, is_capitalized_word,
    obfuscate_uppercase_word, obfuscate_capitalized_word,
};

fn hash_strings(value: &mut Value) {
    match value {
        Value::String(s) => {
            if is_alpha_word(s) {
                *s = hash_word_to_syllables(s);
            } else if is_uppercase_word(s) {
                *s = obfuscate_uppercase_word(s);
            } else if is_capitalized_word(s) {
                *s = obfuscate_capitalized_word(s);
            } else {
                let mut hasher = Sha3_256::new();
                hasher.update(s.as_bytes());
                let result = hasher.finalize();
                *s = hex::encode(result);
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

fn main() {
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
                serde_json::to_writer_pretty(&mut writer, &val).expect("write json");
                writer.write_all(b"\n").expect("newline");
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

    fn sha3_hex(input: &str) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    #[test]
    fn test_hash_strings_simple() {
        let mut value = json!({
            "a": "test",
            "b": ["x", 1],
            "c": {"d": "y"},
            "cap": "Word",
            "u": "UPPER"
        });

        hash_strings(&mut value);

        assert_eq!(value["a"], json!("esal"));
        assert_eq!(value["b"][0], json!("o"));
        assert_eq!(value["b"][1], json!(1));
        assert_eq!(value["c"]["d"], json!("u"));
        assert_eq!(value["cap"], json!("Eqon"));
        assert_eq!(value["u"], json!("AHSON"));
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
        let test_sample: Value =
            serde_json::from_str(TEST_SAMPLE).expect("Failed to parse TEST_SAMPLE");

        let mut hashed_sample = test_sample.clone();
        hash_strings(&mut hashed_sample);
        let hashes = serde_json::to_string_pretty(&hashed_sample)
            .expect("Failed to serialize hashed sample");

        const EXPECTED_HASHES: &str = r#"[
  {
    "additional_information": "4e9be9f98ffaf00dfa6849b118ec0eebaeb9d1fedf49794efc978549d692a644",
    "category": "UMACL",
    "created_at": "cf8cbca8ef96e021217ba62b3f9bc79b3358df6ffabf3036555eb093b6a03900",
    "id": "88cf7ddaff83bfd6f3c9b2f8dfd90987628b01a689b04b0d6f4d6bc05e77c8db",
    "last_edited_by": "972e64ff2f45cb894fd548bbdd0f7d430ba23400502ac9c650d4aa053360ca37",
    "lower case word": "epagrfiovusso",
    "title": "Ylads",
    "updated_at": "cf8cbca8ef96e021217ba62b3f9bc79b3358df6ffabf3036555eb093b6a03900",
    "urls": [
      {
        "href": "d0de71c6aff7c8a492c089fbd5a26a39e76716eef770728c5383386fc245c34b",
        "label": "altysyn",
        "primary": true
      }
    ],
    "vault": {
      "id": "c3427b6423f76857e8ae40651586be4c8bda92ba9c10c201755cb474ea3236d0",
      "name": "Aknbedyip"
    },
    "version": 1
  }
]"#;

        assert_eq!(hashes, EXPECTED_HASHES);
    }
}
