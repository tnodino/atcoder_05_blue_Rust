// https://atcoder.jp/contests/abc062/tasks/arc074_b

use proconio::input;
use proconio::fastout;
use std::cmp::{Reverse, max};
use std::collections::BinaryHeap;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N*3],
    }
    let M = N * 3;
    let mut l = vec![-INF; M];
    let mut s = 0;
    let mut bh = BinaryHeap::new();
    for i in 0..N {
        s += a[i];
        bh.push(Reverse(a[i]));
    }
    l[N-1] = s;
    for i in N..M {
        s += a[i];
        bh.push(Reverse(a[i]));
        s -= bh.pop().unwrap().0;
        l[i] = s;
    }
    let mut r = vec![INF; M];
    let mut s = 0;
    let mut bh = BinaryHeap::new();
    for i in (M-N..M).rev() {
        s += a[i];
        bh.push(a[i]);
    }
    r[M-N] = s;
    for i in (0..M-N).rev() {
        s += a[i];
        bh.push(a[i]);
        s -= bh.pop().unwrap();
        r[i] = s;
    }
    let mut ans = -INF;
    for i in 0..M-1 {
        ans = max(ans, l[i] - r[i+1]);
    }
    println!("{}", ans);
}