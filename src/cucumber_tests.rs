use chrono::TimeZone;
use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
struct TestWorld {
    input: String,
    obfuscated: String,
}

#[given(regex = r#"^the input \"([^\"]+)\"$"#)]
async fn the_input(world: &mut TestWorld, input: String) {
    world.input = input;
}

#[when(regex = r#"^I obfuscate it as (\w+)$"#)]
async fn i_obfuscate_it(world: &mut TestWorld, detector: String) {
    let input_len = world.input.len();
    let hash = crate::shake256_hash(world.input.as_bytes(), input_len.max(32));
    world.obfuscated = match detector.as_str() {
        "alpha_word" => crate::hash_to_syllables(&hash, input_len),
        "uppercase_word" => crate::hash_to_syllables(&hash, input_len).to_ascii_uppercase(),
        "capitalized_word" => {
            let hashed = crate::hash_to_syllables(&hash, input_len);
            if hashed.is_empty() {
                hashed
            } else {
                let mut chars = hashed.chars();
                let first = chars.next().unwrap().to_ascii_uppercase();
                let mut out = String::new();
                out.push(first);
                out.extend(chars);
                out
            }
        }
        "snake_case_word" => crate::hash_to_snake_case(&world.input, &hash),
        "base32_lowercase" => crate::hash_to_base32_lowercase(&hash, input_len),
        "base32_uppercase" => crate::hash_to_base32_uppercase(&hash, input_len),
        "datetime" => {
            let _guard = crate::DATE_TEST_GUARD
                .lock()
                .expect("failed to lock DATE_TEST_GUARD");
            crate::set_date_baselines(chrono::Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap());
            if let Some((datetime, format)) = crate::to_datetime(&world.input) {
                crate::obfuscate_datetime(datetime, format)
            } else {
                world.input.clone()
            }
        }
        "email" => crate::hash_to_email(&hash),
        _ => world.input.clone(),
    };
}

#[then(regex = r#"^the result is a valid (\w+) and equals \"([^\"]+)\"$"#)]
async fn the_result_is(world: &mut TestWorld, detector: String, expected: String) {
    let valid = match detector.as_str() {
        "alpha_word" => crate::is_alpha_word(&world.obfuscated),
        "uppercase_word" => crate::is_uppercase_word(&world.obfuscated),
        "capitalized_word" => crate::is_capitalized_word(&world.obfuscated),
        "snake_case_word" => crate::is_snake_case_word(&world.obfuscated),
        "base32_lowercase" => crate::is_base32_lowercase(&world.obfuscated),
        "base32_uppercase" => crate::is_base32_uppercase(&world.obfuscated),
        "datetime" => crate::to_datetime(&world.obfuscated).is_some(),
        "email" => crate::is_email(&world.obfuscated),
        _ => false,
    };
    assert!(valid, "{} obfuscation failed", detector);
    assert_eq!(world.obfuscated, expected, "unexpected obfuscated output");
}

#[tokio::test]
async fn cucumber_features() {
    TestWorld::cucumber().run("tests/features").await;
}
