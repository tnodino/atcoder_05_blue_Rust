// https://atcoder.jp/contests/abc200/tasks/abc200_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut K: usize,
    }
    let M = N * 3;
    let mut DP: Vec<Vec<isize>> = vec![vec![0; M+1]; 4];
    DP[0][0] = 1;
    for i in 0..3 {
        for j in 0..M {
            DP[i+1][j+1] += DP[i][j];
            if j + N < M {
                DP[i+1][j+N+1] -= DP[i][j];
            }
        }
        for j in 1..=M {
            DP[i+1][j] += DP[i+1][j-1];
        }
    }
    let mut S = 0;
    for i in 0..=M {
        if K <= DP[3][i] as usize {
            S = i;
            break;
        }
        K -= DP[3][i] as usize;
    }
    for x in 1..=N {
        let s = S - x;
        if N * 2 < s {
            continue;
        }
        let y_min;
        if s <= N {
            y_min = 1;
        }
        else {
            y_min = s - N;
        }
        let y_max;
        if N < s {
            y_max = N;
        }
        else {
            y_max = s - 1;
        }
        if y_max - y_min + 1 >= K {
            let y = y_min + K - 1;
            let z = s - y;
            println!("{} {} {}", x, y, z);
            return;
        }
        K -= y_max - y_min + 1;
    }
}