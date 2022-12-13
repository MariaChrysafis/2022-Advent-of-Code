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

fn get_elements(s: Vec<char>) -> Vec<Vec<char>> {
    let mut ans: Vec<Vec<char>> = Vec::new();
    ans.push(Vec::new());
    let mut sum = 1;
    for i in 1..s.len() - 1 {
        if s[i] == '[' {
            sum += 1;
        }
        if s[i] == ']' {
            sum -= 1;
        }
        if sum == 1 && s[i] == ',' {
            ans.push(Vec::new());
            continue;
        }
        let last_index = ans.len() - 1;
        ans[last_index].push(s[i]);
    }
    return ans;
}

fn compare(mut s1: Vec<char>, mut s2: Vec<char>) -> i32 {
    //println!("{:?} {:?}", s1, s2);
    if s1 == s2 {
        return 0;
    }
    if !s1.contains(&'[') && !s2.contains(&'[') {
        if s1 >= s2 {
            return 1;
        }
        if s1 < s2 {
            return -1;
        }
    }
    if s1.len() == 0 {
        return -1;
    }
    if s2.len() == 0 {
        return 1;
    }
    if s1.contains(&'[') && s2.contains(&']') {
        let new_s1 = get_elements(s1.clone());
        let new_s2 = get_elements(s2.clone());
        for i in 0..max(new_s1.len(), new_s2.len()) {
            if i == new_s1.len() {
                return -1;
            }
            if i == new_s2.len() {
                return 1;
            }
            let x = compare(new_s1[i].clone(), new_s2[i].clone());
            if x == -1 || x == 1 {
                return x;
            }
        }
    }
    if !s1.contains(&'[') {
        let mut new_vec1 = vec!['['];
        new_vec1.append(&mut s1);
        new_vec1.push(']');
        return compare(new_vec1.clone(), s2.clone());
    }
    if !s2.contains(&'[') {
        let mut new_vec1 = vec!['['];
        new_vec1.append(&mut s2);
        new_vec1.push(']');
        return compare(s1.clone(), new_vec1.clone());
    }
    return 0;
}

fn main() {
    let mut scan = Scanner::default();
    let mut res: i32 = 0;
    for i in 0.. {
        let mut s1: Vec<char> = scan.next::<String>().chars().collect();
        let mut s2: Vec<char> = scan.next::<String>().chars().collect();
        if compare(s1.clone(), s2.clone()) != 1 {
            res += i + 1;
            println!("{}", res);
        }
        //res += (compare(s1.clone(), s2.clone()) == 1) as i32 * i;
        //println!("{}", res);
        //println!("{}", compare(s1.clone(), s2.clone()));
    }
    println!("{}", res);
}
