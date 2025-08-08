use serde_json::{Deserializer, Value};
use sha3::{Digest, Sha3_256};
use std::io::{self, Write};

mod classifiers;
use classifiers::{
    hash_word_to_syllables, is_alpha_word, is_base32_lowercase, is_capitalized_word,
    is_iso8601_z_datetime, is_snake_case_word, is_uppercase_word,
    obfuscate_base32_lowercase, obfuscate_capitalized_word,
    obfuscate_iso8601_z_datetime, obfuscate_snake_case_word, obfuscate_uppercase_word,
};

fn hash_strings(value: &mut Value) {
    match value {
        Value::String(s) => {
            if is_alpha_word(s) {
                *s = hash_word_to_syllables(s);
            } else if is_snake_case_word(s) {
                *s = obfuscate_snake_case_word(s);
            } else if is_uppercase_word(s) {
                *s = obfuscate_uppercase_word(s);
            } else if is_capitalized_word(s) {
                *s = obfuscate_capitalized_word(s);
            } else if is_iso8601_z_datetime(s) {
                *s = obfuscate_iso8601_z_datetime(s);
            } else if is_base32_lowercase(s) {
                *s = obfuscate_base32_lowercase(s);
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
    use chrono::{TimeZone, Utc};
    use crate::classifiers::{set_date_baselines, DATE_TEST_GUARD};

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

    #[test]
    fn test_hash_strings_simple() {
        let mut value = json!({
            "a": "test",
            "b": ["x", 1],
            "c": {"d": "y"},
            "snake": "snake_case",
            "cap": "Word",
            "u": "UPPER"
        });

        hash_strings(&mut value);

        assert_eq!(value["a"], json!("comi"));
        assert_eq!(value["b"][0], json!("s"));
        assert_eq!(value["b"][1], json!(1));
        assert_eq!(value["c"]["d"], json!("i"));
        assert_eq!(value["cap"], json!("Than"));
        assert_eq!(value["snake"], json!("utcont_stathim"));
        assert_eq!(value["u"], json!("ELIKU"));
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
    "category": "MANNO",
    "created_at": "2000-01-01T00:00:00Z",
    "id": "rdhx3wx7qo75n46jwl4n7wijq5",
    "last_edited_by": "972e64ff2f45cb894fd548bbdd0f7d430ba23400502ac9c650d4aa053360ca37",
    "lower case word": "vericthesneup",
    "title": "Butfa",
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
      "name": "Hedencont"
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
}
