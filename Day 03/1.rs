use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::mem;
fn value (c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        return c as i32 - 'a' as i32 + 1;
    }
    return c as i32 - 'A' as i32 + 26 + 1;
}
fn main() {
    let stdin = io::stdin();
    let mut sum: i32 = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.len() == 0 {
            break;
        }
        let mut oc1: Vec<i32> = vec![0; 53];
        let mut oc2: Vec<i32> = vec![0; 53];
        for (i, c) in s.chars().enumerate() {
            if 2 * i < s.len() {
                //oc1
                oc1[value(c) as usize] += 1;
            } else {
                //oc2
                oc2[value(c) as usize] += 2;
            }
        }
        let mut ans = 0;
        for (i, x) in oc1.iter().enumerate() {
            if *x != 0 && oc2[i] != 0 {
                ans += i;
            }
        }
        sum += ans as i32;
        println!("{}", ans);
    }
    println!("{}", sum);
}
