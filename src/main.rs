mod sample_shell;
use sample_shell::run_shell;
mod excercies;
use excercies::hangman_play;
fn main() {
    // loop {
    //     run_shell();
    // }
    hangman_play();
}
