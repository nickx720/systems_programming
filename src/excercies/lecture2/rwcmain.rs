/// https://github.com/reberhardt7/cs110l-spr-2020-starter-code/blob/main/week2/rwc/src/main.rs
/// https://reberhardt.com/cs110l/spring-2020/assignments/week-2-exercises/
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn open_file(path: &String) -> Result<(usize, usize), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let (mut total_lines, mut word_count) = (0, 0);

    for lines in reader.lines() {
        total_lines += 1;
        word_count += lines?.split_whitespace().collect::<Vec<&str>>().len();
    }
    Ok((total_lines, word_count))
}

pub fn rwcmain() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let (total, word_count) = open_file(filename).unwrap();
    println!("{}    {}      {}", total, word_count, filename);
}
