// https://atcoder.jp/contests/arc023/tasks/arc023_3

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

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

fn nhr(n: usize, r: usize) -> usize {
    let n = n + r - 1;
    let mut num = 1;
    let mut den = 1;
    for i in 1..=r {
        num *= n + 1 - i;
        den *= i;
        num %= MOD;
        den %= MOD;
    }
    return num * bin_power(den, MOD-2) % MOD;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let mut ans = 1;
    let mut l = 0;
    for r in 1..N {
        if A[r] == -1 {
            continue;
        }
        ans *= nhr((A[r] - A[l] + 1) as usize, r - l - 1);
        ans %= MOD;
        l = r;
    }
    println!("{}", ans);
}