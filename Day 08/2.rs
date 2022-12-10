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

fn abs (x: i32) -> i32 {
    if x < 0 {
        return 0-x;
    }
    return x;
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut okay: Vec<Vec<i32>> = Vec::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        vec.push(s.chars().collect());
        okay.push(vec![1; s.len()]);
    }
    for d1 in 0..vec.len() as i32 {
        for d2 in 0..vec[0].len() as i32 {
            for x in -1 .. 2 {
                for y in -1 .. 2 {
                    if (x as i32).abs() + (y as i32).abs() != 1 {
                        continue;
                    }
                    let mut cur_x: i32 = d1.clone();
                    let mut cur_y: i32 = d2.clone();
                    loop {
                        if cur_x + x < 0 || cur_y + y < 0 || cur_x + x == vec.len() as i32 || cur_y + y == vec[0].len() as i32 {
                            break;
                        }
                        cur_x += x;
                        cur_y += y;
                        if vec[cur_x as usize][cur_y as usize] >= vec[d1 as usize][d2 as usize] {
                            break;
                        }
                    }
                    okay[d1 as usize][d2 as usize] *= abs(cur_x - d1) + abs(cur_y - d2);
                }
            }
        }
    }
    let mut ans = 0;
    for x in okay.iter() {
        ans = max(*x.iter().max().unwrap(), ans);
    }
    println!("{}", ans);
}
