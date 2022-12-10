#![allow(warnings)]
#![feature(map_try_insert)]
#![feature(cell_leak)]
#![feature(pattern)]

extern crate core;

use std::borrow::BorrowMut;
use std::cell::{Ref, RefCell};
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::Hash;
use std::io::{self, BufRead};
use std::mem;
use std::rc::{Rc, Weak};
use std::str::pattern::Pattern;
use std::vec;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    pub fn new() -> Location {
        Location { x: 0, y: 0 }
    }
    pub fn overlap(&mut self, head: Location) -> bool {
        if (self.x - head.x).abs() + (self.y - head.y).abs() > 2 {
            return false;
        }
        if (self.x - head.x).abs() == 2 || (self.y - head.y).abs() == 2 {
            return false;
        }
        return true;
    }
    pub fn adjust(&mut self, head: Location) {
        if self.overlap(head.clone()) {
            return;
        }
        if (head.x - self.x).abs() == 0 || (head.y - self.y).abs() == 0 {
            for dx in -1..2 {
                self.x += dx;
                if self.overlap(head.clone()) {
                    return;
                }
                self.x -= dx;
                self.y += dx;
                if self.overlap(head.clone()) {
                    return;
                }
                self.y -= dx;
            }
        }
        if (head.x - self.x).abs() != 0 && (head.y - self.y).abs() != 0 {
            for dx in -1..2 as i32 {
                for dy in -1..2 as i32 {
                    if dx.abs() + dy.abs() == 2 {
                        self.x += dx;
                        self.y += dy;
                        if self.overlap(head.clone()) {
                            return;
                        }
                        self.x -= dx;
                        self.y -= dy;
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut vec: Vec<Vec<char>> = Vec::new();
    let mut okay: Vec<Vec<i32>> = Vec::new();
    let mut head: Vec<Location> = vec![Location::new(); 10];
    let mut mySet: HashSet<(i32, i32)> = HashSet::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            break;
        }
        let vec: Vec<&str> = s.split(' ').collect();
        let dir: String = vec[0].to_string();
        let moves: i32 = vec[1].parse().unwrap();
        for i in 0..moves {
            println!("{} {}", head[1].x, head[1].y);
            mySet.insert((head.last().unwrap().x, head.last().unwrap().y));
            if dir == "R" {
                head[0].x += 1;
            } else if dir == "L" {
                head[0].x -= 1;
            } else if dir == "U" {
                head[0].y += 1;
            } else if dir == "D" {
                head[0].y -= 1;
            }
            for i in 1..head.len() {
                let x = head[i - 1].clone();
                head[i].adjust(x);
            }
            mySet.insert((head.last().unwrap().x, head.last().unwrap().y));
        }
    }
    println!("{}", mySet.len());
}
