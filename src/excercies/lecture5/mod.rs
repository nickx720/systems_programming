// https://youtu.be/NSJW0BvZI0c
use std::{i32, thread, time};
// mod borrow;
fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static + std::ops::Mul,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    // TODO: implement parallel map!
    let mut children = vec![];
    for id in 0..num_threads {
        input_vec.iter().for_each(|&item| {
            children.push(thread::spawn(move || item * item));
        })
    }
    output_vec
}

pub fn parallel_main() {
    //  borrow::borrow_main();
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
