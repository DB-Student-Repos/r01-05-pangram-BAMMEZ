/// Determine whether a sentence is a pangram
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}

