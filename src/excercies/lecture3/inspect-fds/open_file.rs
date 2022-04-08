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
impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Match operators are very commonly used with enums in Rust. They function similar to
        // switch statements in other languages (but can be more expressive).
        match self {
            AccessMode::Read => write!(f, "{}", "read"),
            AccessMode::Write => write!(f, "{}", "write"),
            AccessMode::ReadWrite => write!(f, "{}", "read/write"),
        }
    }
}

/// Stores information about an open file on the system. Since the Linux kernel doesn't really
/// expose much information about the open file table to userspace (cplayground uses a modified
/// kernel), this struct contains info from both the open file table and the vnode table.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenFile {
    pub name: String,
    pub cursor: usize,
    pub access_mode: AccessMode,
}

impl OpenFile {
    #[allow(unused)] // TODO: delete this line for Milestone 4
    pub fn new(name: String, cursor: usize, access_mode: AccessMode) -> OpenFile {
        OpenFile {
            name,
            cursor,
            access_mode,
        }
    }

    /// This function takes a path of an open file and returns a more human-friendly name for that
    /// file.
    ///
    /// * For regular files, this will simply return the supplied path.
    /// * For terminals (files starting with /dev/pts), this will return "<terminal>".
    /// * For pipes (filenames formatted like pipe:[pipenum]), this will return "<pipe #pipenum>".
    #[allow(unused)] // TODO: delete this line for Milestone 4
    fn path_to_name(path: &str) -> String {
        if path.starts_with("/dev/pts/") {
            String::from("<terminal>")
        } else if path.starts_with("pipe:[") && path.ends_with("]") {
            let pipe_num = &path[path.find('[').unwrap() + 1..path.find(']').unwrap()];
            format!("<pipe #{}>", pipe_num)
        } else {
            String::from(path)
        }
    }
