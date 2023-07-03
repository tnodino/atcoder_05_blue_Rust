// https://atcoder.jp/contests/abc007/tasks/abc007_4

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize) -> usize {
    let X = x.to_string().chars().collect::<Vec<char>>();
    let N = X.len();
    let mut DP1 = vec![0; N+1];
    let mut DP2 = vec![0; N+1];
    DP2[0] = 1;
    for i in 0..N {
        DP1[i+1] += DP1[i] * 8;
        let d = (X[i] as usize) - ('0' as usize);
        for j in 0..d {
            if j != 4 && j != 9 {
                DP1[i+1] += DP2[i];
            }
        }
        if d != 4 && d != 9 {
            DP2[i+1] += DP2[i];
        }
    }
    return x - (DP1[N] + DP2[N] - 1);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", f(B) - f(A-1));
}