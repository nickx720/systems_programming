// https://github.com/reberhardt7/cs110l-spr-2020-starter-code/blob/main/week5/farm/src/main.rs
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{env, process, thread};

#[allow(dead_code)]
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for factor in 2..((num as f64).squrt().floor() as u32) {
        if num % factor == 0 {
            return false;
        }
    }
}
