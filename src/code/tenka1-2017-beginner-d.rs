// https://atcoder.jp/contests/tenka1-2017-beginner/tasks/tenka1_2017_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut ans = 0;
    for i in 0..N {
        if K & A[i] == A[i] {
            ans += B[i];
        }
    }
    for b in 1..30 {
        if K & (1 << b) == 0 {
            continue;
        }
        let mut res = 0;
        let k = (K - (1 << b)) | ((1 << b) - 1);
        for i in 0..N {
            if k & A[i] == A[i] {
                res += B[i];
            }
        }
        ans = max(ans, res);
    }
    println!("{}", ans);
}