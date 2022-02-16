fn implement_add_n(vector_of_numbers: Vec<i32>, n: i32) -> Vec<i32> {
    vector_of_numbers.iter().map(|item| item + n).collect()
}
pub fn add_n() {
    println!("{:?}", implement_add_n(vec![1, 2, 3], 2));
}
