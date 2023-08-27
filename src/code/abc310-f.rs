// https://atcoder.jp/contests/abc310/tasks/abc310_f

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const MOD: usize = 998_244_353;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 10;
    let mask = (1 << M) - 1;
    let mut DP = vec![vec![0; 1<<M]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        for bit in 0..1<<M {
            for j in 1..=min(M, A[i]) {
                let bit2 = bit | ((bit << j) & mask) | (1 << (j - 1));
                DP[i+1][bit2] += DP[i][bit] * bin_power(A[i], MOD-2);
                DP[i+1][bit2] %= MOD;
            }
            if A[i] > M {
                DP[i+1][bit] += DP[i][bit] * (A[i] - M) % MOD * bin_power(A[i], MOD-2);
                DP[i+1][bit] %= MOD;
            }
        }
    }
    let mut ans = 0;
    for bit in 0..1<<M {
        if bit & (1 << (M - 1)) > 0 {
            ans += DP[N][bit];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}