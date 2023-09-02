// https://atcoder.jp/contests/abc013/tasks/abc013_3

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        H: isize,
        A: isize,
        B: isize,
        C: isize,
        D: isize,
        E: isize,
    }
    let mut ans = 1<<50;
    for x in 0..=N {
        let y;
        if H + B * x - E * (N - x) > 0 {
            y = 0
        }
        else {
            y = ((E * (N - x) - H - B * x) / (D + E)) + 1;
        }
        ans = min(ans, A * x + C * y);
    }
    println!("{}", ans);
}