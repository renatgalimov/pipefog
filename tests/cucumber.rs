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
    world.obfuscated = match detector.as_str() {
        "alpha_word" => pipefog::hash_word_to_syllables(&world.input),
        "uppercase_word" => pipefog::obfuscate_uppercase_word(&world.input),
        "capitalized_word" => pipefog::obfuscate_capitalized_word(&world.input),
        "snake_case_word" => pipefog::obfuscate_snake_case_word(&world.input),
        "title_case_sentence" => pipefog::obfuscate_title_case_sentence(&world.input),
        "base32_lowercase" => pipefog::obfuscate_base32_lowercase(&world.input),
        "base32_uppercase" => pipefog::obfuscate_base32_uppercase(&world.input),
        _ => world.input.clone(),
    };
}

#[then(regex = r#"^the result is a valid (\w+)$"#)]
async fn the_result_is(world: &mut TestWorld, detector: String) {
    let valid = match detector.as_str() {
        "alpha_word" => pipefog::is_alpha_word(&world.obfuscated),
        "uppercase_word" => pipefog::is_uppercase_word(&world.obfuscated),
        "capitalized_word" => pipefog::is_capitalized_word(&world.obfuscated),
        "snake_case_word" => pipefog::is_snake_case_word(&world.obfuscated),
        "title_case_sentence" => pipefog::is_title_case_sentence(&world.obfuscated),
        "base32_lowercase" => pipefog::is_base32_lowercase(&world.obfuscated),
        "base32_uppercase" => pipefog::is_base32_uppercase(&world.obfuscated),
        _ => false,
    };
    assert!(valid, "{} obfuscation failed", detector);
}

#[tokio::test]
async fn cucumber_features() {
    TestWorld::cucumber().run("tests/features").await;
}
