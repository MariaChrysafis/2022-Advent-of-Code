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

struct Graph {
    adj: Vec<Vec<(usize, usize)>>, //index, weight
}

impl Graph {
    fn new (n: usize) -> Graph {
        Graph {
            adj: vec![vec![]; n],
        }
    }
    fn dijk (&mut self, src: usize) -> Vec<usize> {
        let n = self.adj.len();
        let inf = 1e7 as usize;
        let mut dist: Vec<usize> = vec![inf; n];
        let mut mark: Vec<bool> = vec![false; n];
        dist[src] = 0;
        let mut pq = PriorityQueue::new();
        pq.push(src, 0);
        while !pq.is_empty() {
            let u: usize = *pq.peek().unwrap().0;
            pq.pop();
            if mark[u] {
                continue;
            }
            mark[u] = true;
            for x in self.adj[u].iter() { //from u we go to v
                let v = x.0;
                let w = x.1;
                assert!(w == 1);
                if dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    pq.push(v, inf - dist[v]);
                }
            }
        }
        return dist;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input: Vec<Vec<char>> = Vec::new();
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        input.push(s.chars().collect());
        if s.is_empty() {
            break;
        }
    }
    input.pop();
    let n = input.len();
    let m = input[1].len();
    let mut gr: Graph = Graph::new(n * m);
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for i in 0..n as usize {
        for j in 0..m as usize {
            if input[i][j] == 'S' {
                start = (i, j);
                input[i][j] = 'a';
            }
            if input[i][j] == 'E' {
                end = (i, j);
                input[i][j] = 'z';
            }
        }
    }
    for i in 0..n as i32 {
        for j in 0..m as i32 {
            for dx in -1..2 as i32 {
                for dy in -1..2 as i32 {
                    if dx.abs() + dy.abs() == 1 {
                        if i + dx < 0 || j + dy < 0 || i + dx >= n as i32 || j + dy >= m as i32 {
                            continue;
                        }
                        let new_i = i + dx;
                        let new_j = j + dy;
                        let beginning = input[i as usize][j as usize] as i32;
                        let ending = input[new_i as usize][new_j as usize] as i32;
                        if beginning >= ending - 1 {
                            gr.adj[(i * m as i32 + j) as usize].push(((new_i * m as i32 + new_j) as usize, 1));
                        }
                    }
                }
            }
        }
    }
    println!("{}", gr.dijk(m * start.0 + start.1)[m * end.0 + end.1]);

}
