use std::{collections::HashSet, iter::FromIterator};
fn implement_add_n(vector_of_numbers: Vec<i32>, n: i32) -> Vec<i32> {
    vector_of_numbers.iter().map(|item| item + n).collect()
}

fn implement_add_n_in_place(vector_of_numbers: &mut Vec<i32>, n: i32) {
    for index in 0..vector_of_numbers.len() {
        vector_of_numbers[index] += n;
    }
}
// Implement dedup that removes duplicate elements from a vector in-place (i.e. modifies v directly). If an element is repeated anywhere in the vector, you should keep the element that appears first. You may want to use a HashSet for this.

fn implement_dedup_in_place(vector_of_numbers: Vec<i32>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for item in vector_of_numbers {
        set.insert(item);
    }
    Vec::from_iter(set)
}

pub fn add_n() {
    println!("{:?}", implement_add_n(vec![1, 2, 3], 2));
    let mut sample = vec![2, 3, 4];
    implement_add_n_in_place(&mut sample, 5);
    println!("{:?}", sample);
    let duplicate = vec![1, 1, 1, 2, 2, 3, 4];
    let duplicate = implement_dedup_in_place(duplicate);
    println!("{:?}", duplicate);
}
