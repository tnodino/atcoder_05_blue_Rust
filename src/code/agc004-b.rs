// https://atcoder.jp/contests/agc004/tasks/agc004_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        x: usize,
        mut a: [usize; N],
    }
    let mut ans = a.iter().sum::<usize>();
    for cnt in 1..N {
        let mut b = vec![0; N];
        for i in 0..N {
            b[i] = min(a[i], a[(i+N-1) % N]);
        }
        a = b;
        ans = min(ans, a.iter().sum::<usize>() + x * cnt);
    }
    println!("{}", ans);
}