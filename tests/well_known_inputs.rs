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
        input: "2022-05-16T22:39:20Z",
        detectors: &["datetime"],
    },
    Example {
        input: "2025-10-02 17:41:16+00:00",
        detectors: &["datetime"],
    },
    Example {
        input: "2022-05-16 22:39:20-05:00",
        detectors: &["datetime"],
    },
    Example {
        input: "2026-01-25 14:30:00-0500",
        detectors: &["datetime"],
    },
    Example {
        input: "2025-12-29 13:18:43.470684+00:00",
        detectors: &["datetime"],
    },
    Example {
        input: "2025-12-29T13:18:43.470Z",
        detectors: &["datetime"],
    },
    Example {
        input: "2026-01-25T14:30:00.123456-0500",
        detectors: &["datetime"],
    },
    Example {
        input: "mfrggzdfmztwq2lknnwg23tp",
        detectors: &["base32_lowercase"],
    },
    Example {
        input: "MFRGGZDFMZTWQ2LKNNWG23TP",
        detectors: &["base32_uppercase"],
    },
];
