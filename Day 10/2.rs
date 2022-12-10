#![allow(warnings)]

use std::io::{BufWriter, stdin, stdout, Write};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::collections::vec_deque::VecDeque;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::mem;


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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

struct Register {
    value: i64,
    index: i64,
}

impl Register {
    fn new () -> Register {
        Register {
            value: 1,
            index: 0,
        }
    }
    fn addx (&mut self, val: i64, cycle_count: i64) {
        for i in 0..cycle_count {
            print!("{}", if self.index >= self.value - 1 && self.index <= self.value + 1 { '#' } else { '.' });
            self.index += 1;
            if self.index % 40 == 0 {
                self.index = 0;
                println!();
            }
        }
        self.value += val;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut register: Register = Register::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let vec: Vec<&str> = s.split(' ').collect();
        match vec[0] {
            "addx" => { register.addx(vec[1].parse::<i64>().unwrap(), 2); },
            "noop" => { register.addx(0, 1); },
            _ => { assert!(false); }
        }
    }
}
