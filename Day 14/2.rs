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
    let mut hashSet: HashSet<(i32, i32)> = HashSet::new();
    let mut mx = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let vec: Vec<&str> = s.split(" -> ").collect();
        for i in 1..vec.len() {
            let p: Vec<&str> = vec[i - 1].split(',').collect();
            let c: Vec<&str> = vec[i].split(',').collect();
            let mut prev: (i32, i32) = (p[0].parse().unwrap(), p[1].parse().unwrap());
            let cur: (i32, i32) = (c[0].parse().unwrap(), c[1].parse().unwrap());
            hashSet.insert(cur);
            while prev != cur {
                hashSet.insert(prev);
                prev.0 += Ord::cmp(&cur.0, &prev.0) as i32;
                prev.1 += Ord::cmp(&cur.1, &prev.1) as i32;
                mx = max(mx, prev.1);
                mx = max(mx, cur.1);
            }
        }
    }
    mx += 2;
    for i in 0..100000 {
        hashSet.insert((i, mx));
    }
    for ind in 0..10000000 {
        let mut cur: (i32, i32) = (500, 0);
        loop {
            if !hashSet.contains(&(cur.0, cur.1 + 1)) {
                cur.1 += 1;
                continue;
            }
            if !hashSet.contains(&(cur.0 - 1, cur.1 + 1)) {
                cur.1 += 1;
                cur.0 -= 1;
                continue;
            }
            if !hashSet.contains(&(cur.0 + 1, cur.1 + 1)) {
                cur.1 += 1;
                cur.0 += 1;
                continue;
            }
            break;
        }
        if cur.0 == 500 && cur.1 == 0 {
            println!("DONE {}", ind + 1);
            exit(0);
        }
        hashSet.insert(cur);
    }
    //println!("{} {}", cur.0, cur.1);
}
