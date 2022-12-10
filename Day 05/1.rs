#![allow(warnings)]
/*
Advent of Code 2022 Day 5
*/
use std::cmp::max;
use std::cmp::min;
use std::collections::vec_deque::VecDeque;
use std::io::{self, BufRead};
use std::mem;
use std::vec;
fn main() {
    let stdin = io::stdin();
    let mut v: Vec<VecDeque<char>> = Vec::new();
    let mut moves: bool = false;
    let mut prev: bool = false;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.len() == 0 && prev == true {
            break;
        }
        if s.len() == 0 {
            moves = true;
            prev = true;
            continue;
        }
        prev = false;
        if moves == false && !s.contains("["){
            moves = true;
            continue;
        }
        if moves == false {
            if v.is_empty() {
                v = vec![VecDeque::new(); (s.len() + 1) / 4];
            }
            for (i, c) in s.chars().enumerate() {
                if c == '[' || c == ']' || c == ' ' {
                    continue;
                }
                v[i / 4].push_back(c);
            }
            continue;
        } else {
            let vec: Vec<&str> = s.split(' ').collect();
            let mut num: i32 = vec[1].parse().unwrap();
            let mut i1: usize = vec[3].parse().unwrap();
            let mut i2: usize = vec[5].parse().unwrap();
            i1 -= 1;
            i2 -= 1;
            while num != 0 {
                let x: char = *v[i1].front().unwrap();
                v[i1].pop_front();
                v[i2].push_front(x);
                num -= 1;
            }
            //continue;
        }
    }
    let mut ans: String = String::new();
    for (i, c) in v.iter().enumerate() {
        if c.front() == None {
            continue;
        }
        ans.push(*c.front().unwrap());
    }
    println!("{}", ans);
}
