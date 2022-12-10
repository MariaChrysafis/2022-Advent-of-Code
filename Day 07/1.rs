#![allow(warnings)]
#![feature(map_try_insert)]
#![feature(cell_leak)]
#![feature(pattern)]

extern crate core;

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
use std::cmp::min;
use std::mem;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::str::pattern::Pattern;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn remove (path: &mut String) {
    path.pop();
    while !path.is_empty() && path.chars().last().unwrap() != '/' {
        path.pop();
    }
}

fn derivatives (s: String) -> Vec<String> {
    let mut gamma = s;
    let mut vec: Vec<String> = vec![];
    while gamma.len() != 1 {
        vec.push(gamma.clone());
        remove(&mut gamma);
    }
    return vec;
}

fn main() {
    let stdin = io::stdin();
    let mut path: String = "/".to_string();
    let mut get_files: HashMap<String, (usize, HashSet<String>)> = HashMap::new();
    get_files.try_insert("/".to_string(), (0, HashSet::new()));
    for line in stdin.lock().lines() {
        let s = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let vector: Vec<&str> = s.split(' ').collect();
        if s.starts_with("$ cd ..") {
            remove(&mut path);
        } else if s.starts_with("$ cd /") {
            path = "/".to_string();
        } else if s.starts_with("$ cd ") {
            path.push_str(vector[2]);
            path.push('/');
        } else if s.starts_with("$ ls") {
            continue
        } else if s.starts_with("dir ") {
            let mut x = path.clone();
            x.push_str(&vector[1]);
            x.push('/');
            get_files.insert(x, (0, HashSet::new()));
        } else {
            for x in derivatives(path.clone()) {
                let gamma = get_files.get_mut(&x).unwrap();
                let sz: usize = vector[0].parse().unwrap();
                gamma.1.insert(vector[1].to_string());
                gamma.0 += sz;
            }
        }
    }
    let mut sum = 0;
    for (key, value) in get_files.into_iter() {
        if value.0 <= 100000 {
            sum += value.0;
        }
    }
    println!("{} ", sum);
}
