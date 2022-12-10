#![allow(warnings)]

/*
 * Advent of Code Day 6
*/

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;
fn main() -> io::Result<()> {
    const CONS: usize = 4;
    let file = File::open("/Users/maria/Advent-of-Code/2022/day6/src/input.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let s: String = line.ok().unwrap();
        for (i, c) in s.chars().enumerate() {
            if (HashSet::<char>::from_iter(s[i..i + CONS].to_string().chars().collect::<Vec<char>>().iter().cloned())).len() == CONS {
                println!("{}", i + CONS);
                break;
            }
        }
    }
    Ok(())
}
