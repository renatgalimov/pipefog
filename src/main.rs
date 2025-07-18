use serde_json::{Deserializer, Value};
use sha3::{Digest, Sha3_256};
use std::io::{self, Write};

const SYLLABLES: &[&str] = &[
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

fn hash_strings(value: &mut Value) {
    match value {
        Value::String(s) => {
            let mut hasher = Sha3_256::new();
            hasher.update(s.as_bytes());
            let result = hasher.finalize();
            *s = hex::encode(result);
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
            "c": {"d": "y"}
        });

        hash_strings(&mut value);

        assert_eq!(value["a"], json!(sha3_hex("test")));
        assert_eq!(value["b"][0], json!(sha3_hex("x")));
        assert_eq!(value["b"][1], json!(1));
        assert_eq!(value["c"]["d"], json!(sha3_hex("y")));
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
    "category": "137e3c8495f71ca7e1c165fd3873705cde985f43c13c516a471840f98d472cc6",
    "created_at": "cf8cbca8ef96e021217ba62b3f9bc79b3358df6ffabf3036555eb093b6a03900",
    "id": "88cf7ddaff83bfd6f3c9b2f8dfd90987628b01a689b04b0d6f4d6bc05e77c8db",
    "last_edited_by": "972e64ff2f45cb894fd548bbdd0f7d430ba23400502ac9c650d4aa053360ca37",
    "title": "884e4e4f1742800cbbbb1ffec554ebcd61e8b94cec27ca11efc017c9d582692e",
    "updated_at": "cf8cbca8ef96e021217ba62b3f9bc79b3358df6ffabf3036555eb093b6a03900",
    "urls": [
      {
        "href": "d0de71c6aff7c8a492c089fbd5a26a39e76716eef770728c5383386fc245c34b",
        "label": "f4c0beb05567bed298d8e86439af9c64dbbb86e0804ce527992945db1f873bca",
        "primary": true
      }
    ],
    "vault": {
      "id": "c3427b6423f76857e8ae40651586be4c8bda92ba9c10c201755cb474ea3236d0",
      "name": "2a82dce0734a47938504d6b913fa6c0f05ccefdcc30c96e429f391178770020b"
    },
    "version": 1
  }
]"#;

        assert_eq!(hashes, EXPECTED_HASHES);
    }
}
