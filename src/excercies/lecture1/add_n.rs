fn implement_add_n(vector_of_numbers: Vec<i32>, n: i32) -> Vec<i32> {
    vector_of_numbers.iter().map(|item| item + n).collect()
}

fn implement_add_n_in_place(vector_of_numbers: &mut Vec<i32>, n: i32) {
    for index in 0..vector_of_numbers.len() {
        vector_of_numbers[index] += n;
    }
}

pub fn add_n() {
    println!("{:?}", implement_add_n(vec![1, 2, 3], 2));
    let mut sample = vec![2, 3, 4];
    implement_add_n_in_place(&mut sample, 5);
    println!("{:?}", sample);
}
