extern crate rand;

use std::char;
use rand::distributions::{IndependentSample, Range};

pub fn in_range(start: i32, end: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let valid_range = Range::new(start, end);
    return valid_range.ind_sample(&mut rng);
}

pub fn ascii_string(length: usize) -> String {
    let mut string = String::with_capacity(length);
    let mut i = 0;
    while i < length {
        let c = char::from_u32(in_range('!' as i32, 'z' as i32) as u32);
        if c.is_some() {
            string.push(c.unwrap());
            i += 1;
        }
    }
    return string;
}

pub fn length_ascii_string(start: usize, end: usize) -> String {
    ascii_string(in_range(start as i32, end as i32) as usize)
}
