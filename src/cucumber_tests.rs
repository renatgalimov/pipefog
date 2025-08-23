use cucumber::{given, then, when, World};
use sha3::{Digest, Sha3_256};

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
    let hash = Sha3_256::digest(world.input.as_bytes());
    world.obfuscated = match detector.as_str() {
        "alpha_word" => crate::hash_to_syllables(hash.as_slice(), world.input.len()),
        "uppercase_word" => {
            crate::hash_to_syllables(hash.as_slice(), world.input.len()).to_ascii_uppercase()
        }
        "capitalized_word" => {
            let hashed = crate::hash_to_syllables(hash.as_slice(), world.input.len());
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
        "snake_case_word" => crate::hash_to_snake_case(&world.input, hash.as_slice()),
        "base32_lowercase" => crate::hash_to_base32_lowercase(hash.as_slice(), world.input.len()),
        "base32_uppercase" => crate::hash_to_base32_uppercase(hash.as_slice(), world.input.len()),
        _ => world.input.clone(),
    };
}

#[then(regex = r#"^the result is a valid (\w+)$"#)]
async fn the_result_is(world: &mut TestWorld, detector: String) {
    let valid = match detector.as_str() {
        "alpha_word" => crate::is_alpha_word(&world.obfuscated),
        "uppercase_word" => crate::is_uppercase_word(&world.obfuscated),
        "capitalized_word" => crate::is_capitalized_word(&world.obfuscated),
        "snake_case_word" => crate::is_snake_case_word(&world.obfuscated),
        "base32_lowercase" => crate::is_base32_lowercase(&world.obfuscated),
        "base32_uppercase" => crate::is_base32_uppercase(&world.obfuscated),
        _ => false,
    };
    assert!(valid, "{} obfuscation failed", detector);
}

#[tokio::test]
async fn cucumber_features() {
    TestWorld::cucumber().run("tests/features").await;
}
