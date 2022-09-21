use std::collections::HashMap;

use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::{DwarfData, Error as DwarfError, Line};
use crate::inferior::Inferior;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints: Vec<usize>,
}

#[derive(Clone)]
pub struct Breakpoint {
    addr: usize,
    orig_byte: u8,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        // Print all the breakpoints
        debug_data.print();

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints: vec![],
        }
    }

    pub fn dwarf_get_line_from_addr(&self, value: usize) -> Option<Line> {
        self.debug_data.get_line_from_addr(value)
    }
    pub fn dwarf_get_function_from_addr(&self, value: usize) -> Option<String> {
        self.debug_data.get_function_from_addr(value)
    }

    pub fn get_target(&self) -> &str {
        self.target.as_str()
    }

    fn parse_address(addr: &str) -> Option<usize> {
        let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
            &addr[2..]
        } else {
            &addr
        };
        usize::from_str_radix(addr_without_0x, 16).ok()
    }
    pub fn set_breakpoint(&mut self, breakpoint: usize) {
        self.breakpoints.push(breakpoint);
    }

    pub fn get_breakpoint(&self) -> Vec<usize> {
        self.breakpoints.clone()
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(inferior) = Inferior::new(&self, &args) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        // TODO (milestone 1): make the inferior run
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    return;
                }
                DebuggerCommand::Cont => {
                    if let Some(inferior) = &self.inferior {
                        inferior.continues(self, None);
                    } else {
                        println!("Error resuming process subprocess");
                    }
                }
                DebuggerCommand::Backtrace => {
                    if let Some(inferior) = &self.inferior {
                        inferior.print_backtrace(self);
                    } else {
                        eprintln!("Error backtracing");
                    }
                }
                DebuggerCommand::Break(arg) => {
                    let remove_leading_star = arg.replace("*", "");
                    if let Some(parsed_address) = Self::parse_address(&remove_leading_star) {
                        let count = self.get_breakpoint().len();
                        self.set_breakpoint(parsed_address);

                        println!("Set breakpoint {count} at {arg}");
                    } else {
                        eprintln!("Error parsing");
                    }
                }
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
