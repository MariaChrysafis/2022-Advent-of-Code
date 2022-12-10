use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::mem;
fn main() {
    let stdin = io::stdin();
    let mut ans: i32 = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.len() == 0 {
            break;
        }
        let mut p1 = s.chars().nth(0).unwrap() as i32 - 'A' as i32; //this is opponent
        let mut p2 = s.chars().nth(2).unwrap() as i32 - 'X' as i32; //this is me
        let mut score = p2 + 1;
        if p1 == p2 {
            score += 3;
        } else {
            let mut win = true;
            /*
            0 --> Rock
            1 --> Paper
            2 --> Scissors
             */
            if p1 == 1 && p2 == 0 {
                win = false;
            }
            if p1 == 2 && p2 == 1 {
                win = false;
            }
            if p1 == 0 && p2 == 2 {
                win = false;
            }
            if win {
                score += 6;
            }
        }
        println!("{}", score);
        ans += score;
    }
    println!("{}", ans);
}
