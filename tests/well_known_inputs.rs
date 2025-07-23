pub struct Example {
    pub input: &'static str,
    pub detectors: &'static [&'static str],
}

pub const WELL_KNOWN_INPUTS: &[Example] = &[
    Example {
        input: "lowercase",
        detectors: &["alpha_word"],
    },
    Example {
        input: "UPPERCASE",
        detectors: &["uppercase_word"],
    },
    Example {
        input: "Capitalized",
        detectors: &["capitalized_word"],
    },
    Example {
        input: "snake_case_word",
        detectors: &["snake_case_word"],
    },
    Example {
        input: "A Title Case Sentence",
        detectors: &["title_case_sentence"],
    },
];
