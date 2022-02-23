/// https://github.com/reberhardt7/cs110l-spr-2020-starter-code/blob/main/week2/rdiff/src/main.rs
/// https://reberhardt.com/cs110l/spring-2020/assignments/week-2-exercises/
/// Part 2 Todo rdiff
use super::grid::Grid;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn read_file_lines(path: &String) -> Result<(Vec<String>)> {
    let file = File::open(Path::new(path))?;
    let mut output: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_str = line?;
        output.push(line_str);
    }
    Ok(output)
}
pub fn rdiffmain() {
    let path = String::from("./assets/lecture2/sample.txt");
    read_file_lines(&path);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("./assets/lecture2/handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }
}
