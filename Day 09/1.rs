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
#[derive(Clone)]
struct Location {
    x: i32,
    y: i32,
}
impl Location {
    pub fn new () -> Location {
        Location {
            x: 0,
            y: 0,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut okay: Vec<Vec<i32>> = Vec::new();
    let mut head: Location = Location::new();
    let mut tail: Location = Location::new();
    println!("{}", tail.x);
    let mut mySet: HashSet<(i32, i32)> = HashSet::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let vec: Vec<&str> = s.split(' ').collect();
        let dir: String = vec[0].to_string();
        let moves: i32 = vec[1].parse().unwrap();
        println!("{}", moves);
        for i in 0..moves {
            mySet.insert((tail.x, tail.y));
            println!("{} {}", tail.x, tail.y);
            if dir == "R" {
                head.x -= 1;
            } else if dir == "L" {
                head.x += 1;
            } else if dir == "U" {
                head.y -= 1;
            } else if dir == "D" {
                head.y += 1;
            }
            if (tail.x - head.x).abs() + (tail.y - head.y).abs() == 1 {
                continue;
            }
            let mut fine: bool = false;
            if (head.x - tail.x).abs() != (head.y - tail.y).abs() {
                for dx in -1..2 {
                    tail.x += dx;
                    if (tail.x - head.x).abs() + (tail.y - head.y).abs() == 1 {
                        fine = true;
                        break;
                    }
                    tail.x -= dx;
                }
                for dy in -1..2 {
                    tail.y += dy;
                    if (tail.x - head.x).abs() + (tail.y - head.y).abs() == 1 {
                        fine = true;
                        break;
                    }
                    tail.y -= dy;
                }
            }
            if fine {
                continue;
            }
            for dx in -1..2 as i32 {
                for dy in -1..2 as i32 {
                    if dx.abs() + dy.abs() == 2 {
                        tail.x += dx;
                        tail.y += dy;
                        if (tail.x - head.x).abs() + (tail.y - head.y).abs() == 1 {
                            fine = true;
                            break;
                        }
                        tail.x -= dx;
                        tail.y -= dy;
                    }
                }
                if fine {
                    break;
                }
            }
        }
    }
    println!("{}", mySet.len());
}
