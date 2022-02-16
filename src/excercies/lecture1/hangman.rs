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

fn hangman(container: WordToCheck, mut display: String) -> io::Result<(i32, String)> {
    let WordToCheck { word, mut attempts } = container;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let response = buffer.trim();
    if !word.contains(response) {
        attempts -= 1;
        io::stdout().write_all(format!("Wrong, you have {} left\n", attempts).as_bytes())?;
        return Ok((attempts, display));
    };
    let first = response
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap()
        .to_owned();
    let mut display_container = display.chars().collect::<Vec<char>>();
    for (index, item) in word.chars().enumerate() {
        if item == first {
            display_container[index] = item;
            continue;
        }
    }
    display = display_container.iter().cloned().collect::<String>();

    Ok((attempts, display))
}

pub fn warhammer() {
    println!("Welcome to hangman");
    println!("Guess the word");
    let mut random = thread_rng();
    let word = ANSWERS.iter().choose(&mut random).unwrap();
    let mut display = "_".repeat(word.len());
    let mut container = WordToCheck::new(word, 5);

    while container.attempts != 0 {
        println!("\n{},", display);
        let response = hangman(container, display).unwrap();
        container.attempts = response.0;
        display = response.1;
    }
    println!("Game over");
}
