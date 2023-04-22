// https://atcoder.jp/contests/arc027/tasks/arc027_3

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        N: usize,
    }
    let mut DP = vec![vec![vec![0; Y+1]; X+1]; N+1];
    for i in 0..N {
        input! {
            t: usize,
            h: usize,
        }
        for x in 0..=X {
            for y in 0..=Y {
                DP[i+1][x][y] = max(DP[i+1][x][y], DP[i][x][y]);
                if x >= 1 && x + y >= t {
                    let a;
                    let b;
                    if t <= y {
                        a = x - 1;
                        b = y - (t - 1);
                    }
                    else {
                        a = x - (t - y);
                        b = 0;
                    }
                    DP[i+1][a][b] = max(DP[i+1][a][b], DP[i][x][y] + h);
                }
            }
        }
    }
    println!("{}", DP[N][0][0]);
}