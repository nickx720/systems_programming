use std::io::{self, Read, Write};

fn hangman(mut attempts: i32) -> io::Result<(i32)> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    if buffer.trim() == "yes" {
        attempts -= 1;
    }
    io::stdout().write_all(buffer.to_uppercase().as_bytes())?;
    Ok(attempts)
}

pub fn warhammer() {
    println!("Welcome");
    let mut attempts = 5;
    while attempts != 0 {
        attempts = hangman(attempts).unwrap();
    }
    println!("Game over");
}
