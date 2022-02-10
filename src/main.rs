mod sample_shell;
use sample_shell::run_shell;
fn main() {
    loop {
        run_shell();
    }
}
