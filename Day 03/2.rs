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
fn get_oc (s: String) -> Vec<i32> {
    let mut oc: Vec<i32> = vec![0; 53];
    for (i, c) in s.chars().enumerate() {
        oc[value(c) as usize] += 1;
    }
    return oc;
}
fn main() {
    let stdin = io::stdin();
    let mut sum: i32 = 0;
    let mut vec: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        let s = line.unwrap().to_string();
        if s.len() == 0 {
            break;
        }
        vec.push(s);
    }
    for (i, x) in vec.iter().enumerate() {
        if 3 * i + 2 > vec.len() {
            continue;
        }
        let oc1 = get_oc(vec.get(3 * i).unwrap().to_string());
        let oc2 = get_oc(vec.get(3 * i + 1).unwrap().to_string());
        let oc3 = get_oc(vec.get(3 * i + 2).unwrap().to_string());
        for (i, x) in oc1.iter().enumerate() {
            if oc1[i] != 0 && oc2[i] != 0 && oc3[i] != 0 {
                sum += i as i32;
            }
        }
        println!("{}", sum);
    }
    println!("{}", sum);
}
