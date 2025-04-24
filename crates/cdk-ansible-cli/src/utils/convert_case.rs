use convert_case::Boundary;

/// ex)
/// - `AAbb` -> `a_abb` (boundary)
/// - `AAb1` -> `aab1` (no boundary) (ex: `IPv4` -> `ipv4` with [`Boundary::LOWER_DIGIT`])
pub const ACRONYM_WITH_TWO_LOWER: Boundary = Boundary {
    name: "AcronymWithTwoLower",
    condition: |s, _| {
        s.get(0).map(grapheme_is_uppercase) == Some(true)
            && s.get(1).map(grapheme_is_uppercase) == Some(true)
            && s.get(2).map(grapheme_is_lowercase) == Some(true)
            && s.get(3).map(grapheme_is_lowercase) == Some(true)
    },
    arg: None,
    start: 1,
    len: 0,
};

// MIT License
//
// Copyright (c) 2025 rutrum
fn grapheme_is_uppercase(c: &&str) -> bool {
    c.to_uppercase() != c.to_lowercase() && *c == c.to_uppercase()
}

// MIT License
//
// Copyright (c) 2025 rutrum
fn grapheme_is_lowercase(c: &&str) -> bool {
    c.to_uppercase() != c.to_lowercase() && *c == c.to_lowercase()
}
