use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[allow(unused_imports)] // TODO: delete this line for Milestone 4
use std::{fmt, fs};

#[allow(unused)] // TODO: delete this line for Milestone 4
const O_WRONLY: usize = 00000001;
#[allow(unused)] // TODO: delete this line for Milestone 4
const O_RDWR: usize = 00000002;
#[allow(unused)] // TODO: delete this line for Milestone 4
const COLORS: [&str; 6] = [
    "\x1B[38;5;9m",
    "\x1B[38;5;10m",
    "\x1B[38;5;11m",
    "\x1B[38;5;12m",
    "\x1B[38;5;13m",
    "\x1B[38;5;14m",
];

#[allow(unused)] // TODO: delete this line for Milestone 4
const CLEAR_COLOR: &str = "\x1B[0m";

/// This enum can be used to represent whether a file is read-only, write-only, or read/write. An
/// enum is basically a value that can be one of some number of "things."
#[allow(unused)] // TODO: delete this line for Milestone 4
#[derive(Debug, Clone, PartialEq)]
pub enum AccessMode {
    Read,
    Write,
    ReadWrite,
}
