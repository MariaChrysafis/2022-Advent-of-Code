#![allow(warnings)]

use std::io::{BufWriter, stdin, stdout, Write};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::collections::vec_deque::VecDeque;
use std::fs::soft_link;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::mem;
use std::process::exit;
use std::ptr::hash;
extern crate meval;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Faild read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<(i32, i32, i32)> = Vec::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let gamma: Vec<&str> = s.split(',').collect();
        vec.push((gamma[0].parse::<i32>().unwrap(), gamma[1].parse::<i32>().unwrap(), gamma[2].parse::<i32>().unwrap()));
        //println!("{}", s);
    }
    let mut ans = 0;
    for i in 0..vec.len() {
        let mut c: i32 = 6;
        for j in 0..vec.len() {
            if (vec[i].0 - vec[j].0).abs() + (vec[i].1 - vec[j].1).abs() + (vec[i].2 - vec[j].2).abs() != 1 {
                continue;
            }
            c -= 1;
        }
        ans += c;
    }
    println!("{}", ans);

}
