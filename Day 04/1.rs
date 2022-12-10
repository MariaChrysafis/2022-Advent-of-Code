#![allow(warnings)]
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::mem;
fn inside_range (l: i32, r: i32, x: i32) -> bool {
    return (x >= l && x <= r);
}
fn main() {
    let stdin = io::stdin();
    let mut sum: i32 = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string(); //test
        if s.len() == 0 {
            break;
        }
        let mut v = Vec::new(); 
        for s in s.split(',').collect::<Vec<&str>>() {
            for x in s.split('-').collect::<Vec<&str>>() {
                v.push(x.parse::<i32>().unwrap());
            }
        }
        if (inside_range(v[0], v[1], v[2]) && inside_range(v[0], v[1], v[3])) || (inside_range(v[2], v[3], v[0]) && inside_range(v[2], v[3], v[1])) {
            sum += 1;
        }
    }
    println!("{}", sum);
}
