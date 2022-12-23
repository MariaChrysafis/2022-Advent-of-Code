#![allow(warnings)]

use std::io::{BufWriter, stdin, stdout, Write};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
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

fn to_move (cubes: HashSet<(i32, i32, i32)>, position: (i32, i32, i32), dxyz: (i32, i32, i32)) -> (i32, i32, i32) {
    let mut cur = position.clone();
    while !cubes.contains(&cur) {
        cur.0 += dxyz.0;
        cur.1 += dxyz.1;
        cur.2 += dxyz.2;
    }
    return cur;
}

fn get_adj (coord: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut q: Vec<(i32, i32, i32)> = Vec::new();
    q.push((coord.0 - 1, coord.1, coord.2));
    q.push((coord.0 + 1, coord.1, coord.2));
    q.push((coord.0, coord.1 - 1, coord.2));
    q.push((coord.0, coord.1 + 1, coord.2));
    q.push((coord.0, coord.1, coord.2 - 1));
    q.push((coord.0, coord.1, coord.2 + 1));
    return q;
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<(i32, i32, i32)> = Vec::new();
    let mut mx = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let gamma: Vec<&str> = s.split(',').collect();
        vec.push((gamma[0].parse::<i32>().unwrap() + 1, gamma[1].parse::<i32>().unwrap() + 1, gamma[2].parse::<i32>().unwrap() + 1));
        mx = max(mx, max(max(vec.last().unwrap().0, vec.last().unwrap().2), vec.last().unwrap().1));
    }
    mx += 1;
    println!("{}", mx);
    let mut cubes: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
    for x in vec.iter() {
        cubes.insert(*x);
    }
    let mut vis: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
    let mut q: Vec<(i32, i32, i32)> = Vec::new();
    q.push((mx, mx, mx));
    while !q.is_empty() {
        let coord = *(q.last().unwrap());
        q.pop();
        if vis.contains(&coord) || cubes.contains(&coord) {
            continue;
        }
        if coord.0 < 0 || coord.1 < 0 || coord.2 < 0 || coord.0 > mx || coord.1 > mx || coord.2 > mx {
            continue;
        }
        vis.insert(coord);
        for alpha in get_adj(coord).iter() {
            q.push(*alpha);
        }
    }
    let mut c = 0;
    for x in 0..vec.len() {
        for alpha in get_adj(vec[x]).iter() {
            if vis.contains(alpha) {
                c += 1;
            }
        }
    }
    println!("{}", c);

}
