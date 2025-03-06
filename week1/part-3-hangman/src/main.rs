// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    //println!("random word: {}", secret_word);

    // Your code here! :)
    let mut current_guess: Vec<char> = vec!['-'; secret_word.len()];
    let mut guessed_letters: Vec<char> = Vec::new();

    println!("Welcome to CS110L Hangman!");

    let mut num_guesses = NUM_INCORRECT_GUESSES;
    while num_guesses > 0 {
        println!(
            "The word so far is: {}",
            current_guess.iter().collect::<String>()
        );
        println!(
            "You have guessed the following letters: {}",
            guessed_letters.iter().collect::<String>()
        );
        println!("You have {} guesses left", num_guesses);
        print!("Please guess a letter: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: char = guess.trim().chars().next().unwrap();
        if !guessed_letters.contains(&guess) {
            guessed_letters.push(guess);
            num_guesses -= 1;
        };

        if secret_word.contains(guess) {
            for (i, &c) in secret_word_chars.iter().enumerate() {
                if c == guess {
                    current_guess[i] = c;
                }
            }
        } else {
            println!("Sorry, that letter is not in the word")
        }

        if !current_guess.contains(&'-') {
            println!("Congratulations, you guessed the word: {}", secret_word);
            return;
        }

        println!();
    }

    println!(
        "Sorry, you ran out of guesses! The word was: {}",
        secret_word
    );
}
