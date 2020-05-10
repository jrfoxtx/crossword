//
// /Users/Jim/projects/functions/src/main.rs
//
use regex::Regex;
use std::collections::HashSet;
use std::env;

const MAX_LETTERS: usize = 7;
const MIN_LETTERS: usize = 3;

const DEFAULT_EXCLUSIONS: &str = "\
^b[bcdfgkmnpqstvxz]|^c[bcdfgkmnpqstvx]|^d[bcdfgklmnpqstvxz]|^f[bcdfgkmnpqstvxz]|\
^g[cdfgkmpqstvxz]|^h[bcdfghkmnpqtvxz]|^j[bcdfgjklmnpqrstvxz]|^k[bcdfgkmpqstxz]|\
^l[cdfgkmnpqrstvxz]|^m[bcdfgkmpqrstvxz]|^mn[^e]|^n[bcdfghjklmnpqrstvxz]|^p[bcdgkmpqvwxz]\
^q[bcdfghjklmnpqrstvxz]|^r[bcdfgklmnpqstvwxz]|^s[dgrx]|^t[bcdfgkmnpqtvx]|\
^v[bcdfghkmnpqstvxz]|^w[bcdfgjklmnpqstvwxz]|^wr[^aeiouy]|^x[bcdfgklmnpqrstvxz]|^y[bcdfghjklmnpqstvwxz]^z[bcdfgknpqtvxz]\
[cdfgjknpqstvxz]b$|[bdfghjkpqtvxyz]c$|[bcfgjkpqtvxyz]d$|[bcdgjkqstvxz]f$|[cdfjkpqstvxz]g$|\
[flmnqvwxz]h$|[cdfghklmnpqstvxz]j$|[bdfgjmpqtvxz]k$|[bcdfgjkmnpqstvxz]l$|[bcdfgnpqtvxz]m$|\
[bcdfjkpqstvxz]n$|[bcdfghjknqtvxz]p$|[^a]q$|[bcdfgjklmnpqstvxz]r$|[bgjqvxz]t$|[bcdfghjkmnpqstvwxz]v$|\
[bcdfghjklmnpqrstvwxz]w$|[^aeiouy]x$|[bcdfghkpqvx]z$\
[aeiou]{4,}|a{3}|b{3,}|d{3,}|e{3}|i{3}|o{3}|p{3,}|s{3,}|t{3,}|u{3}";

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
    exclusions: Option<regex::Regex>,
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
        for c in &pattern {
            if *c != b'.' {
                assert!(pool.contains(&c));
                let index = pool.iter().position(|l : &u8| l == c).unwrap();
                pool.remove(index);
            }
        }
        pool.sort();
        let pool = pool.to_vec();

        let mut exclusions : Option<Regex> = None;
        if args.len() >= 4 {
            if args[3].len() > 0 {
                let rx_result = Regex::new(&args[3]);
                match rx_result {
                    Ok(rx) => {
                        println!("Using supplied exclusions: '{}'", args[3]);
                        exclusions = Some(rx);
                    },
                    Err(err) => {
                        eprintln!("Could not compile RegEx from '{}': {}", args[3], err);
                    },
                }
            } else {
                println!("Using a blank argument to skip default exclusions; no exclusions used.");
            }
        } else {
            let rx_result = Regex::new(DEFAULT_EXCLUSIONS);
            match rx_result {
                Ok(rx) => {
                    println!("Using supplied exclusions: '{}'", DEFAULT_EXCLUSIONS);
                    exclusions = Some(rx);
                },
                Err(err) => {
                    eprintln!("Could not compile RegEx from '{}': {}", DEFAULT_EXCLUSIONS, err);
                },
            }
    }
        Ok(Config{pattern, pool, exclusions})
    }
}

fn add_if_not_match(word: String, words: &mut HashSet<String>, exclusions: &Option<Regex>) {
    if !words.contains(&word) {
        match exclusions {
            Some(exclusions) => {
                if !exclusions.is_match(&word) {
                    words.insert(word);
                }
            },
            None => {
                words.insert(word);
            }
        }
    }
}

fn fill_letter(level: usize, template: &Vec<u8>, pool: &Vec<u8>, letters: &mut Vec<u8>, bookmarks: &mut Vec<usize>, exclusions: &Option<Regex>, words: &mut HashSet<String>) {
    if template[level] != b'.' {
        letters.push(template[level]);
        if level + 1 < template.len() {
            fill_letter(level + 1, template, pool, letters, bookmarks, exclusions, words);
        } else {
            let word: String = String::from_utf8(letters.clone()).expect("All letters are ASCII");
            add_if_not_match(word, words, exclusions);
        }
        letters.pop();
    } else {
        for i in 0..pool.len() {
            if !bookmarks.contains(&i) {
                letters.push(pool[i]);
                if level + 1 < template.len() {
                    bookmarks.push(i);
                    fill_letter(level + 1, template, pool, letters, bookmarks, exclusions, words);
                    bookmarks.pop();
                } else {
                    let word: String = String::from_utf8(letters.clone()).expect("All letters are ASCII");
                    add_if_not_match(word, words, exclusions);
                }
                letters.pop();
            }  // else this member of |pool| has already been used.
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).expect("Insufficient arguments");
    unsafe {
        eprintln!("Pattern   : {}", String::from_utf8_unchecked(config.pattern.to_vec()));
        eprintln!("Pool      : {}", String::from_utf8_unchecked(config.pool.to_vec()));
        if args.len() > 3 {
            eprintln!("Exclusions: {}", args[3]);
        }
    }

    let mut non_dot_count = 0;
    for c in &config.pattern {
        if *c != b'.' {
            non_dot_count = non_dot_count + 1;
        }
    }

    // if args.len() > 3 {
    //     let r = Regex::new(&args[3]).expect("Must be legal expression");
    //     if r.is_match("ab") {
    //         eprintln!("Test Regex matches.");
    //     } else {
    //         eprintln!("Test Regex does not match.");
    //     }
    // }

    eprintln!("The number of pre-specified letters is: {}; {}! = {}", non_dot_count, non_dot_count, factorial(non_dot_count));
    // println!("The maximum number of possible words in a    game is: {}", factorial(MAX_LETTERS));
    // println!("The maximum number of possible words in this game is: {}", factorial(config.pattern.len()));
    // println!("The maximum number of legal    words in this game is: {}", factorial(config.pattern.len()) / factorial(non_dot_count));
    let mut words : HashSet<String> = HashSet::new();
    let mut bookmarks: Vec<usize> = Vec::with_capacity(MAX_LETTERS);
    let mut letters: Vec<u8> = Vec::with_capacity(MAX_LETTERS);
    fill_letter(0usize, &config.pattern, &config.pool, &mut letters, &mut bookmarks, &config.exclusions, &mut words);
    let mut word_list: Vec<String> = words.into_iter().collect();
    word_list.sort();
    for word in word_list {
        println!("{}", word);
    }
}
