use rand::{prelude::IteratorRandom, thread_rng};
use std::io::{self, Write};

const ANSWERS: [&'static str; 5] = ["dog", "cat", "monkey", "spiderman", "chicago"];

#[derive(Clone, Copy)]
struct WordToCheck {
    word: &'static str,
    attempts: i32,
}
impl WordToCheck {
    fn new(word: &'static str, attempts: i32) -> Self {
        Self { word, attempts }
    }
}

fn hangman(mut container: WordToCheck) -> io::Result<(i32)> {
    let WordToCheck { word, mut attempts } = container;
    println!("{}", &word);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let response = buffer.trim();
    dbg!(word.contains(response));
    if !word.contains(response) {
        attempts -= 1;
    };
    io::stdout().write_all("_".repeat(word.len()).as_bytes())?;
    Ok(attempts)
}

pub fn warhammer() {
    println!("Welcome to hangman");
    println!("Guess the word");
    let mut random = thread_rng();
    let word = ANSWERS.iter().choose(&mut random).unwrap();
    let mut container = WordToCheck::new(word, 5);

    let underscore = "_".repeat(container.word.len());
    println!("{}", underscore);
    while container.attempts != 0 {
        container.attempts = hangman(container).unwrap();
    }
    println!("Game over");
}
