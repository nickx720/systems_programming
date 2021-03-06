// https://github.com/reberhardt7/cs110l-spr-2020-starter-code
// Grid implemented as a flat vector
pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    elems: Vec<usize>,
}

impl Grid {
    /// Returns a Grid of the specified size, with all the elements pre-initalized to zero.
    pub fn new(num_rows: usize, num_cols: usize) -> Grid {
        Grid {
            num_rows,
            num_cols,
            /// This systax uses the vec! macro to create a vector of zeros, initialized to a
            /// specific length
            elems: vec![0; num_rows * num_cols],
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.num_rows, self.num_cols)
    }

    // Fix index
    fn index(&self, row: usize, col: usize) -> usize {
        let response = row * self.num_cols + col;
        if response >= self.num_rows * self.num_cols {
            return response - 1;
        }
        response
    }

    /// Returns the element at the specified location. If the location is out of bounds, return
    /// None.
    ///
    /// Note to students: this function also could have returned Result. It's a matter of taste in
    /// how you define the semantics; many languages raise exceptions for out-of-bounds exceptions,
    /// but others argue that makes code needlessly complex. Here, we decided to return Option to
    /// give you more practice with Option :) and because this similar library returns Option:
    /// https://docs.rs/array2d/0.2.1/array2d/struct.Array2D.html
    pub fn get(&self, row: usize, col: usize) -> Option<usize> {
        let (cur_row, cur_col) = self.size();
        let index = cur_row * cur_col;
        let get_index = self.index(row, col);
        if get_index >= index {
            return None;
        }
        Some(self.elems[get_index])
    }

    /// Sets the element at the specified location to the specified value. If the location is out
    /// of bounds, returns Err with an error message.
    pub fn set(&mut self, row: usize, col: usize, val: usize) -> Result<(), &'static str> {
        let (cur_row, cur_col) = self.size();
        let length = cur_row * cur_col;
        let index = self.index(row, col);
        if index >= length {
            return Err("Something went wrong");
        }
        self.elems[index] = val;
        Ok(())
    }

    /// Prints a visual representation of the grid. You can use this for debugging.
    pub fn display(&self) {
        for row in 0..self.num_rows {
            let mut line = String::new();
            for col in 0..self.num_cols {
                line.push_str(&format!("{}, ", self.get(row, col).unwrap()));
            }
            println!("{}", line);
        }
    }

    /// Resets all the elements to zero
    pub fn clear(&mut self) {
        for i in self.elems.iter_mut() {
            *i = 0;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid() {
        let n_rows = 4;
        let n_cols = 3;
        let mut grid = Grid::new(n_rows, n_cols);

        // Initialize grid
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.set(r, c, r * n_cols + c).is_ok(),
                    "Grid::set returned Err even though the provided bounds are valid!"
                );
            }
        }

        // Note: you need to run "cargo test  -- --nocapture" in order to see output printed
        println!("Grid contents:");
        grid.display();

        // Make sure the values are what we expect
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.get(r, c).is_some(),
                    "Grid::get returned None even though the provided bounds are valid!"
                );
                assert_eq!(grid.get(r, c).unwrap(), r * n_cols + c);
            }
        }
    }
}
