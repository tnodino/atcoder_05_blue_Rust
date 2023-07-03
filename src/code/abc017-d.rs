// https://atcoder.jp/contests/abc017/tasks/abc017_4

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        f: [usize; N],
    }
    let mut DP = vec![0; N+1];
    DP[0] = 1;
    let mut flg = vec![false; M];
    let mut S = 1;
    let mut l = 0;
    for r in 0..N {
        while flg[f[r]-1] {
            flg[f[l]-1] = false;
            S = (S + MOD - DP[l]) % MOD;
            l += 1;
        }
        flg[f[r]-1] = true;
        DP[r+1] = S;
        S += DP[r+1];
        S %= MOD;
    }
    println!("{}", DP[N]);
}