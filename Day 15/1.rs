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
use priority_queue::PriorityQueue;

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
    let mut hs: HashSet<(i32, i32)> = HashSet::new();
    let mut to_remove: Vec<(i32, i32)> = Vec::new();
    for line in stdin.lock().lines() {
        let mut s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        s = s.replace("Sensor at ", "");
        s = s.replace(": closest beacon is at", "");
        s = s.replace("x=", "");
        s = s.replace("y=", "");
        s = s.replace(",", "");
        let vector: Vec<&str> = s.split(' ').collect();
        let mut keys: Vec<i32> = Vec::new();
        for i in vector.iter() {
            println!("{}", *i);
            keys.push((*i).parse::<i32>().unwrap());
        }
        to_remove.push((keys[0], keys[1]));
        to_remove.push((keys[2], keys[3]));
        let dist: i32 = (keys[0] - keys[2]).abs() + (keys[1] - keys[3]).abs();
        let row = 2000000;
        for i in keys[0] - dist..keys[0] + dist + 1 {
            let j = row;
            if (keys[0] - i).abs() + (keys[1] - j).abs() > dist {
                continue;
            }
            hs.insert((i, j));
        }
        println!("{:?}", vector);
    }
    for x in to_remove.iter() {
        hs.remove(x);
    }
    println!("{}", hs.len());

}
