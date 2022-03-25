use crate::open_file::OpenFile;
#[allow(unused)] // TODO: delete this line for Milestone 3
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct Process {
    pub pid: usize,
    pub ppid: usize,
    pub command: String,
}

impl Process {
    pub fn new(pid: usize, ppid: usize, command: String) -> Process {
        Process { pid, ppid, command }
    }

    /// This function returns a list of file descriptor numbers for this Process, if that
    /// information is available (it will return None if the information is unavailable). The
    /// information will commonly be unavailable if the process has exited. (Zombie processes
    /// still have a pid, but their resources have already been freed, including the file
    /// descriptor table.)
    #[allow(unused)] // TODO: delete this line for Milestone 3
    pub fn list_fds(&self) -> Option<Vec<usize>> {
        // TODO: implement for Milestone 3
     if let Ok(home_dir) = env::var("HOME"){
           let this_pid = self.pid;
           let dir = format!("/proc/{this_pid}/fd");
           let entries = fs::read_dir(dir).ok()?
               .map(|res| res.map(|entry| entry.file_name()))
               .map(|item| {
                   if let Some(response)= item.to_str(){
                       response.to_string()
                   }
               })
               .collect::<Result<Vec<_>,io::Error>>().ok()?;
           let string_entries = entries.iter().map(|item| {
               if let Some(response) = item.to_str(){
                   response.to_string()
               } else {
                   String::new()
               }          }).collect::<Vec<String>>();
           dbg!(string_entries);

        unimplemented!();
    }

    /// This function returns a list of (fdnumber, OpenFile) tuples, if file descriptor
    /// information is available (it returns None otherwise). The information is commonly
    /// unavailable if the process has already exited.
    #[allow(unused)] // TODO: delete this line for Milestone 4
    pub fn list_open_files(&self) -> Option<Vec<(usize, OpenFile)>> {
        let mut open_files = vec![];
        for fd in self.list_fds()? {
            open_files.push((fd, OpenFile::from_fd(self.pid, fd)?));
        }
        Some(open_files)
    }
}
