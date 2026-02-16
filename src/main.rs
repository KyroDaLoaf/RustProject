use rand::prelude::IndexedRandom;
use rand::rng;
use std::collections::HashSet;
use std::io::{self, Write};

fn read_line_trimmed() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn mask_word(word: &str, guessed: &HashSet<char>) -> String {
    word.chars()
        .map(|c| {
            if c == ' ' {
                ' '
            } else if guessed.contains(&c.to_ascii_lowercase()) {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn all_revealed(word: &str, guessed: &HashSet<char>) -> bool {
    word.chars()
        .all(|c| c == ' ' || guessed.contains(&c.to_ascii_lowercase()))
}

fn main() {
    let words: [&str; 8] = [
        "rust",
        "computer",
        "borrow checker",
        "lifetime",
        "compiler",
        "cargo",
        "trait",
        "pattern",
    ];

    let mut r = rng();
    let word: &str = *words.choose(&mut r).unwrap();

    let mut guessed: HashSet<char> = HashSet::new();
    let mut wrong: HashSet<char> = HashSet::new();
    let mut attempts_left: i32 = 6;

    println!("Hangman (console)");
    println!("Guess letters. You have {} wrong attempts.", attempts_left);

    loop {
        let masked = mask_word(word, &guessed);

        println!(
            "\nWord: {}",
            masked
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        println!("Wrong: {}", {
            let mut v: Vec<char> = wrong.iter().copied().collect();
            v.sort_unstable();
            v.into_iter().collect::<String>()
        });

        println!("Attempts left: {}", attempts_left);

        if all_revealed(word, &guessed) {
            println!("\nYou win! The word was: {}", word);
            break;
        }

        if attempts_left <= 0 {
            println!("\nYou lose! The word was: {}", word);
            break;
        }

        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let input = read_line_trimmed().to_ascii_lowercase();

        let ch_opt = input.chars().find(|c| c.is_ascii_alphabetic());
        let Some(ch) = ch_opt else {
            println!("Please enter a valid letter.");
            continue;
        };

        if guessed.contains(&ch) || wrong.contains(&ch) {
            println!("You already tried '{}'.", ch);
            continue;
        }

        if word.to_ascii_lowercase().contains(ch) {
            guessed.insert(ch);
            println!("Good guess!");
        } else {
            wrong.insert(ch);
            attempts_left -= 1;
            println!("Nope!");
        }
    }
}
