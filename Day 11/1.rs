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
extern crate meval;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

struct Monkey {
    items: Vec<i32>,
    operation: String,
    test: i32,
    if_true: usize,
    if_false: usize,
    inspect: usize,
}

impl Monkey {
    fn new () -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: "".to_string(),
            if_true: 0,
            test: 0,
            if_false: 0,
            inspect: 0,
        }
    }
    fn print (&mut self) {
        println!("test: {:?}, items: {:?}; operation: {}; if_true: {}; if_false: {}",self.test, self.items, self.operation, self.if_true, self.if_false);
    }
}

pub fn operation (x: &mut i32, s: String) -> i32 {
    let mut inp = s.clone();
    let gamma = (*x).to_string();
    inp = inp.replace("old", &gamma);
    return meval::eval_str(inp).unwrap() as i32;
}

fn main() {
    let stdin = io::stdin();
    let mut monkeys: Vec<Monkey> = vec![Monkey::new()];
    let mut prev: bool = false;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if prev == true && s.is_empty() {
            break;
        }
        prev = s.is_empty();
        if s.is_empty() {
            monkeys.push(Monkey::new());
            continue;
        }
        if &s[0..1] == "M" {
            continue;
        }
        let last_index = monkeys.len() - 1;
        let vec: Vec<&str> = s.split(' ').collect();
        if &s[2..3] == "S" {
            let gamma = (&s[18..s.len()]).to_string();
            let new_vec: Vec<&str> = gamma.split(", ").collect();
            for i in new_vec.iter() {
                monkeys[last_index].items.push(i.parse::<i32>().unwrap());
            }
        } else if &s[2..3] == "O" {
            let gamma = (&s[19..s.len()]).to_string();
            monkeys[last_index].operation = gamma;
        } else if &s[2..3] == "T" {
            let gamma = (&s[21..s.len()]).to_string();
            monkeys[last_index].test = gamma.parse::<i32>().unwrap();
        } else if &s[7..8] == "t" {
            let gamma = *vec.last().unwrap();
            monkeys[last_index].if_true = gamma.parse::<usize>().unwrap();
        } else if &s[7..8] == "f" {
            let gamma = *vec.last().unwrap();
            monkeys[last_index].if_false = gamma.parse::<usize>().unwrap();
        }
    }
    monkeys.pop();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut last_item = *monkeys[i].items.last().unwrap();
                last_item = operation(&mut last_item, monkeys[i].operation.clone());
                last_item /= 3;
                let b: bool = (last_item % monkeys[i].test == 0);
                let x: usize = match b {
                    true => monkeys[i].if_true,
                    false => monkeys[i].if_false
                };
                monkeys[i].inspect += 1;
                monkeys[x].items.push(last_item);
                monkeys[i].items.pop();
            }
        }
    }
    let mut vec: Vec<usize> = Vec::new();
    for monkey in monkeys.iter_mut() {
        vec.push(monkey.inspect);
    }
    vec.sort();
    vec.reverse();
    println!("{}", vec[0] * vec[1]);
}
