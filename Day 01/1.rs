use std::io::{self, BufRead};
use std::vec;
use std::cmp::max;
fn main() {
    let stdin = io::stdin();
    let mut res: i32 = 0;
    let mut ans: i32 = 0;
    for line in stdin.lock().lines() {
        let s: String = line.unwrap().to_string();
        if s.is_empty() {
            if res == 0 {
                break;
            }
            res = 0;
            continue;
        }
        let my_int = s.parse::<i32>().unwrap();
        res += my_int;
        ans = max(ans, res);
    }
    println!("{}", ans);
}
