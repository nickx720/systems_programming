use std::{
    io::{self, Write},
    process,
};

/// flushes text buffer to the stdout

fn write_to_stdout(text: &str) -> io::Result<()> {
    io::stdout().write(text.as_ref())?;
    io::stdout().flush()?;
    Ok(())
}

fn get_user_command() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.ends_with("\n") {
        input.pop();
    }
    input
}

pub fn run_shell() {
    let shellname = "ghost#";
    match write_to_stdout(&shellname) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Unable to write to stdout: {}", e);
            process::exit(1);
        }
    }
    let cmd = get_user_command();
    if let Err(_) = process::Command::new(&cmd).status() {
        eprintln!("{ }: command not found", &cmd);
    }
}
