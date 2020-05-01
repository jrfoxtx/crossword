//
// /Users/Jim/projects/functions/src/main.rs
//

use std::env;
// use regex::Regex;

const MAX_LETTERS: usize = 7;
const LETTER_BUFFER_SIZE: usize = MAX_LETTERS + 1;
const MIN_LETTERS: usize = 3;

fn factorial(n: usize) -> usize {
    match n {
      0 => { 1 }
      _ => { n * factorial(n - 1) }
    }
}

#[derive(Debug)]
struct Config {
    pattern: [u8; LETTER_BUFFER_SIZE],
    pool: [u8; LETTER_BUFFER_SIZE],
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Need arguments *pattern* and *letters*");
        }

        let pattern_string = args[1].clone().trim().to_ascii_lowercase();
        if !args[1].is_ascii() || pattern_string.len() < MIN_LETTERS || pattern_string.len() > MAX_LETTERS {
            return Err("The supplied pattern must be between 3 and 7 ASCII letters (or period) long.")
        }

        let pool_string = args[2].clone().trim().to_ascii_lowercase();
        if !args[2].is_ascii() || pool_string.len() < MIN_LETTERS || pool_string.len() > MAX_LETTERS {
            return Err("The supplied possible letters must be between 3 and 7 ASCII letters long.")
        }
        if pattern_string.len() > pool_string.len() {
            return Err("The supplied pattern cannot be longer than the supplied possible letters");
        }

        let pattern_bytes_temp = pattern_string.as_bytes();
        let mut pattern: [u8; LETTER_BUFFER_SIZE] = [0u8; LETTER_BUFFER_SIZE];
        for i in 0..pattern_bytes_temp.len() {
            if pattern_bytes_temp[i] == 0u8 {
                break;
            }
            pattern[i] = pattern_bytes_temp[i];
        }

        let mut pool_bytes_temp = pool_string.as_bytes().clone();
        pool_bytes_temp.sort();
        let mut pool: [u8; LETTER_BUFFER_SIZE] = [0u8; LETTER_BUFFER_SIZE];
        for i in 0..pool_bytes_temp.len() {
            if pool_bytes_temp[i] == 0u8 {
                break;
            }
            pool[i] = pool_bytes_temp[i];
        }

        Ok(Config{pattern, pool})
    }
}

fn fill_letter(level: usize, template: &[u8; LETTER_BUFFER_SIZE], pool: &[u8; LETTER_BUFFER_SIZE], letters: &mut [u8; LETTER_BUFFER_SIZE], bookmarks: &mut[usize; MAX_LETTERS], indices: &mut Vec<std::str::CharIndices> ) {
    println!("level: {}, letters: {:?}, bookmarks: {:?}", level, letters, bookmarks);
    for c in &letters {
        letters[level] = *c;
        if level + 1 < template.len() {
            fill_letter(level + 1, template, pool, letters, bookmarks, indices);
        } else {
            println!("Level {}: letters: {}", level, String::from_utf8(letters).expect("All characters are ASCII"));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).expect("Insufficient arguments");
    println!("Parsed arguments: {:?}", config);

    println!("The maximum number of possible words in a game is: {}", factorial(MAX_LETTERS));

    let pattern_string = String::from_utf8(config.pattern.to_vec()).expect("Pattern is always ASCII.");
    let pattern_string_length = pattern_string.len();
    println!("pattern: {} ({})", pattern_string, pattern_string_length);
    println!("The number of possible words matching pattern {} is: {}", pattern_string, factorial(pattern_string_length));

    let mut pool_string_vector = config.pattern.to_vec();
    let pool_string = String::from_utf8(config.pattern.to_vec()).expect("Pool is always ASCII.");
    let pool_string_length = pool_string.len();
    pool_string_vector.sort();
    pool_string_vector.dedup();
    let pool_string_dedup = String::from_utf8(pool_string_vector).expect("Pool is always ASCIId.");
    let pool_string_dedup_length = pool_string_dedup.len();
    println!("pool: {} ({})", pool_string, pool_string_length);
    println!("pool_dedup: {} ({})", pool_string_dedup, pool_string_dedup_length);

    let mut indices: Vec<std::str::CharIndices> = vec![pool_string.char_indices(); MAX_LETTERS];
    let mut bookmarks: [usize; MAX_LETTERS] = [0usize; MAX_LETTERS];
    let mut letters: [u8; LETTER_BUFFER_SIZE] = [0u8; LETTER_BUFFER_SIZE];
    let pattern: [u8; LETTER_BUFFER_SIZE] = [0u8; LETTER_BUFFER_SIZE];
    let pool: [u8; LETTER_BUFFER_SIZE] = [0u8; LETTER_BUFFER_SIZE];

    println!("There are {} iterators and {} bookmarks", indices.len(), bookmarks.len());

    fill_letter(0usize, &pattern, &pool, &mut letters, &mut bookmarks, &mut indices);
}
