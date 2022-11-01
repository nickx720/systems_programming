// https://github.com/reberhardt7/cs110l-spr-2020-starter-code/blob/main/week5/farm/src/main.rs
use num_cpus;
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
use std::time::Instant;
#[allow(unused_imports)]
use std::{env, process, thread};

#[allow(dead_code)]
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for factor in 2..((num as f64).sqrt().floor() as u32) {
        if num % factor == 0 {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn factor_number(num: u32) {
    let start = Instant::now();

    if num == 1 || is_prime(num) {
        println!("{} = {} [time: {:?}]", num, num, start.elapsed());
        return;
    }

    let mut factors = Vec::new();
    let mut curr_num = num;
    for factor in 2..num {
        while curr_num % factor == 0 {
            factors.push(factor);
            curr_num /= factor;
        }
    }
    factors.sort();
    let factors_str = factors
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" * ");
    println!("{} = {} [time: {:?}]", num, factors_str, start.elapsed());
}

pub fn factor_prime_main() {
    let args: Vec<Vec<u32>> = env::args()
        .skip(1)
        .map(|item| {
            item.split("")
                .map(|item| item.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|items| {
            items
                .into_iter()
                .map(|item| item.parse::<u32>())
                .filter_map(|item| item.ok())
                .collect::<Vec<u32>>()
        })
        .collect();
    dbg!(args);
    let num_threads = num_cpus::get();
    println!("Farm starting on {} CPUs", num_threads);
    let start = Instant::now();
    // TODO: call get_input_numbers() and store a queue of numbers to factor

    // TODO: spawn `num_threads` threads, each of which pops numbers off the queue and calls
    // factor_number() until the queue is empty

    // TODO: join all the threads you created
    println!("Total execution time: {:?}", start.elapsed());
}
