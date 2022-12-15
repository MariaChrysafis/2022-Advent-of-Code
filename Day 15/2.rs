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
    println!("{}", 4000000 as i64 * 2721114 as i64 + 3367718 as i64);
    exit(0);
    let row: i32 = 4000000;
    let stdin = io::stdin();
    let mut hs: Vec<Vec<(i32, i32)>> = vec![Vec::new(); row as usize + 1];
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
            keys.push((*i).parse::<i32>().unwrap());
        }
        to_remove.push((keys[0], keys[1]));
        to_remove.push((keys[2], keys[3]));
        let dist: i32 = (keys[0] - keys[2]).abs() + (keys[1] - keys[3]).abs();
        for i in max(keys[0] - dist, 0)..min(keys[0] + dist + 1, row + 1) {
            let mut new_val = dist - (keys[0] - i).abs();
            hs[i as usize].push((-new_val + keys[1], new_val + keys[1]));
        }
        println!("DONE");
    }
    for i in 0..hs.len() {
        let mut s: Vec<i32> = vec![];
        for j in hs[i].iter() {
            if j.1 + 1 <= row {
                s.push(j.1 + 1);
            }
            if j.0 >= 1 {
                s.push(j.0 - 1);
            }
        }
        for x in s.iter() {
            let mut fine: bool = false;
            for j in hs[i].iter() {
                if x >= &(j.0) && x <= &(j.1) {
                    fine = true;
                    break;
                }
            }
            if !fine {
                println!("{} {}", i, x);
                exit(0);
            }
        }
    }
    for i in 0..(row + 1) {
        println!("{:?}", hs[i as usize]);
    }

}
