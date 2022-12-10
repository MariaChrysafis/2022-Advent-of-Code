use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::os::unix::raw::uid_t;

fn main() {
    let stdin = io::stdin();
    let mut ans: i32 = 0;
    let mut vec: Vec<i32> = Vec::new();
    vec.push(0);
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            if vec.last() == Some(&0) {
                break;
            }
            vec.push(0);
            continue;
        }
        if vec.is_empty() {
            vec.push(0);
        }
        let mut my_int: i32 = s.parse::<i32>().unwrap();
        let mut i: usize = vec.len() - 1;
        vec[i] += my_int;
    }
    vec.sort();
    vec.reverse();
    println!("{}", vec[0] + vec[1] + vec[2]);
}
