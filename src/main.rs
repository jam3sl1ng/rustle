use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const ALL_WORDS: &str = include_str!("assets/words.txt");
const WORD_LENGTH: usize = 5;
const MAX_ATTEMPTS: usize = 6;

// Format a word by removing whitespace, converting to uppercase, and filtering out non-alphabetic characters.
fn clean_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()  
}

// Examines he words.txt file and returns a list of words
fn words_list() -> Vec<String> {
    ALL_WORDS
        .split('\n')
        .map(clean_word)
        .filter(|line| line.len() == WORD_LENGTH)
        .collect()
}

fn main() {
    println!("Hello, world!");
}
