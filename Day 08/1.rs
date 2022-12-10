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
        okay.push(vec![0; s.len()]);
    }
    for x in -1..2 {
        for y in -1..2 {
            if (x as i32).abs() + (y as i32).abs() != 1 {
                continue;
            }
            for d1 in 0..vec.len() as i32 {
                for d2 in 0..vec[0].len() as i32 {
                    if d1 != 0 && d1 != vec.len() as i32 - 1 && d2 != 0 && d2 != vec[0].len() as i32 - 1 {
                        continue;
                    }
                    for fin_x in 0..vec.len() as i32 {
                        for fin_y in 0..vec[0].len() as i32 {
                            let mut x_coord = d1.clone();
                            let mut y_coord = d2.clone();
                            let mut pos: bool = false;
                            while true {
                                if x_coord == fin_x && y_coord == fin_y {
                                    pos = true;
                                    break;
                                }
                                if x_coord + x < 0 || y_coord + y < 0 || x_coord + x >= vec.len() as i32 || y_coord + y >= vec[0].len() as i32 {
                                    pos = false;
                                    break;
                                }
                                if vec[x_coord as usize][y_coord as usize] >= vec[fin_x as usize][fin_y as usize] {
                                    break;
                                }
                                x_coord += x as i32;
                                y_coord += y as i32;
                            }
                            println!("{} {} {}", fin_x, fin_y, pos);
                            if pos {
                                okay[fin_x as usize][fin_y as usize] = 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for (a, x) in okay.iter().enumerate() {
        sum += x.iter().sum::<i32>();
    }
    println!("{}", sum);
}
