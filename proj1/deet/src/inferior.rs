use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::mem::size_of;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};

use crate::debugger::Debugger;
use crate::dwarf_data::Line;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
    ///indicates the process has been restarted
    Restart,
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(debugger: &Debugger, args: &Vec<String>) -> Option<Inferior> {
        let target = debugger.get_target();
        let child_process = unsafe {
            let child_process = Command::new(target)
                .args(args)
                .pre_exec(child_traceme)
                .spawn()
                .expect(format!("{target} failed to execute process").as_str());
            child_process
        };
        let mut inferior = Inferior {
            child: child_process,
        };
        let breakpoint = debugger.get_breakpoint();
        // When creating an Inferior, you should pass Inferior::new a list of breakpoints. In Inferior::new, after you wait for SIGTRAP (indicating that the inferior has fully loaded) but before returning, you should install these breakpoints in the child process.
        if !breakpoint.is_empty() {
            for (index, &item) in breakpoint.iter().enumerate() {
                let value = inferior
                    .write_byte(item, index as u8)
                    .expect("couldn't set breakpoint");
            }
        }
        let status = inferior.continue_exec(debugger);
        if status.is_ok() {
            return Some(inferior);
        } else {
            println!(
                "Inferior::new not implemented! target={}, args={:?}",
                target, args
            );
            None
        }
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    /// Continue method
    pub fn continue_exec(&self, debugger: &Debugger) -> Result<Status, nix::Error> {
        ptrace::cont(self.pid(), None);
        let status = self.wait(None);
        match status {
            Ok(resp) => match resp {
                Status::Signaled(signal) => println!("{signal}"),
                Status::Stopped(signal, reg) => {
                    // Setting up breakpoint
                    if signal == nix::sys::signal::SIGTRAP {
                        println!("This is a sigtrap");
                    }
                    eprintln!("Child stopped (signal {signal})");
                    let file_name = debugger.dwarf_get_line_from_addr(reg).expect(
                        "Line
                    not available",
                    );
                    eprintln!("Stopped at {file_name}");
                }
                Status::Exited(code) => println!("Child Exited (status {code})"),
                _ => eprint!("Paniced"),
            },
            Err(e) => eprint!("{e}"),
        }
        Ok(Status::Restart)
    }

    fn print_function_line_no(line_number: Option<Line>, file_name: &Option<String>) {
        match (line_number, file_name) {
            (Some(line), Some(file)) => println!("{file} ({line})"),
            (None, None) => eprintln!("Nothing to show"),
            _ => unreachable!(),
        };
    }

    pub fn print_backtrace(&self, debugger: &Debugger) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        let (mut instruction_ptr, mut base_ptr) = (regs.rip as usize, regs.rbp as usize);
        loop {
            let line_number = debugger.dwarf_get_line_from_addr(instruction_ptr);
            let file_name = debugger.dwarf_get_function_from_addr(instruction_ptr);
            Inferior::print_function_line_no(line_number, &file_name);
            if file_name.unwrap() == "main" {
                break;
            }
            instruction_ptr =
                ptrace::read(self.pid(), (base_ptr + 8) as ptrace::AddressType)? as usize;
            base_ptr = ptrace::read(self.pid(), base_ptr as ptrace::AddressType)? as usize;
        }
        Ok(())
    }

    fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }
}
