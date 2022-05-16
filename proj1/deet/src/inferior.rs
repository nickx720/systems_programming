use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};

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

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>) -> Option<Inferior> {
        let child_process = unsafe {
            let child_process = Command::new(target)
                .args(args)
                .pre_exec(child_traceme)
                .spawn()
                .expect(format!("{target} failed to execute process").as_str());
            child_process
        };
        let inferior = Inferior {
            child: child_process,
        };
        let status = inferior.continue_exec();
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
    pub fn continue_exec(&self) -> Result<Status, nix::Error> {
        ptrace::cont(self.pid(), None);
        let status = self.wait(None);
        match status {
            Ok(resp) => match resp {
                Status::Signaled(signal) => println!("{signal}"),
                Status::Stopped(signal, _reg) => println!("Child stopped with (signal {signal})"),
                Status::Exited(code) => println!("Child Exited (status {code})"),
                _ => eprint!("Paniced"),
            },
            Err(e) => eprint!("{e}"),
        }
        Ok(Status::Restart)
    }

    pub fn print_backtrace(&self) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        println!("Hello World");
        Ok(())
    }
}
