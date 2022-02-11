use std::io::{self, Read, Write};

fn hangman() -> io::Result<()> {
    let mut buffer = "".to_string();
    io::stdin().read_to_string(&mut buffer)?;
    dbg!(&buffer);
    io::stdout().write_all(buffer.to_uppercase().as_bytes())?;
    Ok(())
}

pub fn warhammer() {
    println!("Welcome");
    loop {
        hangman().unwrap();
    }
}
