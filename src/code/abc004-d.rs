// https://atcoder.jp/contests/abc004/tasks/abc004_4

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        G: usize,
        B: usize,
    }
    let N = R + G + B;
    let M = 2_000;
    let mut DP = vec![vec![1<<32; N+1]; M];
    for i in 0..M {
        DP[i][0] = 0;
    }
    for i in 1..M {
        for j in 1..=N {
            let x;
            if j <= R {
                x = (i as isize - 900).abs() as usize;
            }
            else if j <= R + G {
                x = (i as isize - 1_000).abs() as usize;
            }
            else {
                x = (i as isize - 1_100).abs() as usize;
            }
            DP[i][j] = min(DP[i-1][j], DP[i-1][j-1] + x);
        }
    }
    println!("{}", DP[M-1][N]);
}