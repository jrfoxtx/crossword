//
// /Users/Jim/projects/functions/src/main.rs
//
use std::env;

// use regex::Regex;

const MAX_LETTERS: usize = 7;
const MIN_LETTERS: usize = 3;

fn factorial(n: usize) -> usize {
    match n {
      0 => { 1 }
      _ => { n * factorial(n - 1) }
    }
}

#[derive(Debug)]
struct Config {
    pattern: Vec<u8>,
    pool: Vec<u8>,
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

        let pattern = pattern_string.as_bytes().to_vec();
        let mut pool = pool_string.as_bytes().to_vec();
        pool.sort();
        let pool = pool.to_vec();
        Ok(Config{pattern, pool})
    }
}

fn fill_letter(level: usize, template: &Vec<u8>, pool: &Vec<u8>, letters: &mut Vec<u8>, bookmarks: &mut Vec<usize>) {
    // println!("level: {}, letters: {:?}, bookmarks: {:?}, template.len(): {}", level, letters, bookmarks, template.len());
    if level >= template.len() {
        println!("fill_letter: reached end of word.");
        letters.push(0u8);
        let word: String = String::from_utf8(letters.clone()).expect("All letters are ASCII");
        println!("{}; level: {}; bookmarks = {:?}", word, level, bookmarks);
        letters.pop();
    } else {
        // println!("fill_letter: need to add {} more letters", template.len() - letters.len());
        for i in 0..pool.len() {
            // println!("fill_letter: trying letter '{}' at index {}", pool[i], i);
            if !bookmarks.contains(&i) {
                // println!("fill_letter: bookmarks {:?} do not contain {}", bookmarks, i);
                letters.push(pool[i]);
                if level + 1 < template.len() {
                    bookmarks.push(i);
                    fill_letter(level + 1, template, pool, letters, bookmarks);
                    bookmarks.pop();
                } else {
                    // println!("fill_letter: reached end of word.");
                    letters.push(0u8);
                    let word: String = String::from_utf8(letters.clone()).expect("All letters are ASCII");
                    println!("{}; level: {}; bookmarks = {:?}", word, level, bookmarks);
                    letters.pop();
                }
                letters.pop();
            }  // else this member of |pool| has already been used.
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).expect("Insufficient arguments");
    println!("Parsed arguments: {:?}", config);
    unsafe {
        println!("Pattern: {}", String::from_utf8_unchecked(config.pattern.to_vec()));
        println!("Pool   : {}", String::from_utf8_unchecked(config.pool.to_vec()));
    }
    println!("The maximum number of possible words in a    game is: {}", factorial(MAX_LETTERS));
    println!("The maximum number of possible words in this game is: {}", factorial(config.pattern.len()));

    let mut bookmarks: Vec<usize> = Vec::new();
    let mut letters: Vec<u8> = Vec::new();
    fill_letter(0usize, &config.pattern, &config.pool, &mut letters, &mut bookmarks);
}
