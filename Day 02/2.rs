use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::mem;
fn res (p1: i32, p2: i32) -> i32 {
    //-1 for loss
    //0 for tie
    //1 for win
    if p1 == p2 {
        return 0; //tie
    }
    if p1 == 1 && p2 == 0 {
        return -1;
    }
    if p1 == 2 && p2 == 1 {
        return -1;
    }
    if p1 == 0 && p2 == 2 {
        return -1;
    }
    return 1;
}
fn main() {
    let stdin = io::stdin();
    let mut ans: i32 = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.len() == 0 {
            break;
        }
        let mut p1 = s.chars().nth(0).unwrap() as i32 - 'A' as i32; //this is opponent
        let mut p2 = s.chars().nth(2).unwrap() as i32 - 'X' as i32; //this is my round result
        p2 -= 1;
        let mut score = 0;
        score += 3 * (p2 + 1);
        for i in 0..3 {
            if res(p1, i) == p2 {
                score += i + 1;
            }
        }
        println!("{}", score);
        ans += score;
    }
    println!("{}", ans);
}
