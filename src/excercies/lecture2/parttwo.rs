/// https://github.com/reberhardt7/cs110l-spr-2020-starter-code/blob/main/week2/rdiff/src/main.rs
/// https://reberhardt.com/cs110l/spring-2020/assignments/week-2-exercises/
/// Part 2 Todo rdiff
use super::grid::Grid;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn read_file_lines(path: &String) -> Result<Vec<String>> {
    let file = File::open(Path::new(path))?;
    let mut output: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        let line_str = line?;
        output.push(line_str);
    }
    Ok(output)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let row = seq1.len() + 1;
    let col = seq2.len() + 1;
    let mut output: Grid = Grid::new(row, col);
    for i in 1..row {
        for j in 1..col {
            if seq1[i - 1] == seq2[j - 1] {
                if let Some(current_val) = output.get(i - 1, j - 1) {
                    let _ = output.set(i, j, current_val + 1);
                }
            } else {
                let (prev_val, curr_val) = (output.get(i, j - 1), output.get(i - 1, j));
                if let Some(prev_val) = prev_val {
                    if let Some(curr_val) = curr_val {
                        let max_val = std::cmp::max(prev_val, curr_val);
                        let _ = output.set(i, j, max_val);
                    }
                }
            }
        }
    }
    output
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    unimplemented!();
    // Be sure to delete the #[allow(unused)] line above
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

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
